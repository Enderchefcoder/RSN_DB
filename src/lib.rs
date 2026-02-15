use pyo3::exceptions::{PyIOError, PyKeyError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
enum DbError {
    #[error("table `{0}` does not exist")]
    MissingTable(String),
    #[error("field `{0}` is missing")]
    MissingField(String),
    #[error("field `{0}` must be unique")]
    UniqueViolation(String),
    #[error("record id `{0}` does not exist")]
    MissingRecord(u64),
    #[error("schema type mismatch for field `{field}`: expected `{expected}`")]
    TypeMismatch { field: String, expected: String },
    #[error("io error: {0}")]
    Io(String),
}

type DbResult<T> = Result<T, DbError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FieldDef {
    field_type: FieldType,
    required: bool,
    unique: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    Json,
}

impl FieldType {
    fn from_str(raw: &str) -> Option<Self> {
        match raw.to_ascii_lowercase().as_str() {
            "string" | "str" | "text" => Some(Self::String),
            "integer" | "int" => Some(Self::Integer),
            "float" | "double" | "number" => Some(Self::Float),
            "boolean" | "bool" => Some(Self::Boolean),
            "json" | "object" => Some(Self::Json),
            _ => None,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Integer => "integer",
            Self::Float => "float",
            Self::Boolean => "boolean",
            Self::Json => "json",
        }
    }

    fn matches(&self, value: &Value) -> bool {
        match self {
            Self::String => value.is_string(),
            Self::Integer => value.as_i64().is_some() || value.as_u64().is_some(),
            Self::Float => value.is_number(),
            Self::Boolean => value.is_boolean(),
            Self::Json => true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Table {
    schema: HashMap<String, FieldDef>,
    records: HashMap<u64, Map<String, Value>>,
    next_id: u64,
    indexes: Vec<String>,
}

impl Table {
    fn new(schema: HashMap<String, FieldDef>) -> Self {
        Self {
            schema,
            records: HashMap::new(),
            next_id: 1,
            indexes: Vec::new(),
        }
    }

    fn validate_payload(
        &self,
        payload: &Map<String, Value>,
        updating: Option<u64>,
    ) -> DbResult<()> {
        for (field, def) in &self.schema {
            if def.required && !payload.contains_key(field) {
                return Err(DbError::MissingField(field.clone()));
            }

            if let Some(value) = payload.get(field) {
                if !value.is_null() && !def.field_type.matches(value) {
                    return Err(DbError::TypeMismatch {
                        field: field.clone(),
                        expected: def.field_type.label().to_string(),
                    });
                }
            }

            if def.unique {
                if let Some(candidate) = payload.get(field) {
                    for (record_id, record) in &self.records {
                        if Some(*record_id) == updating {
                            continue;
                        }

                        if let Some(existing) = record.get(field) {
                            if existing == candidate {
                                return Err(DbError::UniqueViolation(field.clone()));
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn insert(&mut self, payload: Map<String, Value>) -> DbResult<u64> {
        self.validate_payload(&payload, None)?;
        let record_id = self.next_id;
        self.next_id += 1;
        self.records.insert(record_id, payload);
        Ok(record_id)
    }

    fn update(&mut self, record_id: u64, patch: Map<String, Value>) -> DbResult<()> {
        let Some(existing) = self.records.get(&record_id).cloned() else {
            return Err(DbError::MissingRecord(record_id));
        };

        let mut merged = existing;
        for (key, value) in patch {
            merged.insert(key, value);
        }

        self.validate_payload(&merged, Some(record_id))?;
        self.records.insert(record_id, merged);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Engine {
    tables: HashMap<String, Table>,
}

impl Engine {
    fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    fn create_table(&mut self, name: &str, schema: HashMap<String, FieldDef>) {
        self.tables.insert(name.to_string(), Table::new(schema));
    }

    fn table(&self, name: &str) -> DbResult<&Table> {
        self.tables
            .get(name)
            .ok_or_else(|| DbError::MissingTable(name.to_string()))
    }

    fn table_mut(&mut self, name: &str) -> DbResult<&mut Table> {
        self.tables
            .get_mut(name)
            .ok_or_else(|| DbError::MissingTable(name.to_string()))
    }
}

fn value_cmp(left: &Value, right: &Value) -> Ordering {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => {
            let aa = a.as_f64().unwrap_or_default();
            let bb = b.as_f64().unwrap_or_default();
            aa.partial_cmp(&bb).unwrap_or(Ordering::Equal)
        }
        (Value::String(a), Value::String(b)) => a.cmp(b),
        (Value::Bool(a), Value::Bool(b)) => a.cmp(b),
        _ => Ordering::Equal,
    }
}

fn py_to_json(value: Bound<'_, PyAny>) -> PyResult<Value> {
    if value.is_none() {
        return Ok(Value::Null);
    }

    if let Ok(raw) = value.extract::<bool>() {
        return Ok(Value::Bool(raw));
    }
    if let Ok(raw) = value.extract::<i64>() {
        return Ok(Value::Number(raw.into()));
    }
    if let Ok(raw) = value.extract::<f64>() {
        if let Some(number) = serde_json::Number::from_f64(raw) {
            return Ok(Value::Number(number));
        }
    }
    if let Ok(raw) = value.extract::<String>() {
        return Ok(Value::String(raw));
    }
    if let Ok(list) = value.downcast::<PyList>() {
        let mut out = Vec::with_capacity(list.len());
        for item in list {
            out.push(py_to_json(item)?);
        }
        return Ok(Value::Array(out));
    }
    if let Ok(dict) = value.downcast::<PyDict>() {
        let mut out = Map::new();
        for (k, v) in dict.iter() {
            let key = k.extract::<String>()?;
            out.insert(key, py_to_json(v)?);
        }
        return Ok(Value::Object(out));
    }

    Err(PyValueError::new_err(
        "unsupported value type: only scalar/list/dict/json-compatible values are allowed",
    ))
}

fn json_to_py(py: Python<'_>, value: &Value) -> PyResult<PyObject> {
    Ok(match value {
        Value::Null => py.None(),
        Value::Bool(v) => v.into_py(py),
        Value::Number(v) => {
            if let Some(number) = v.as_i64() {
                number.into_py(py)
            } else if let Some(number) = v.as_u64() {
                number.into_py(py)
            } else {
                v.as_f64().unwrap_or_default().into_py(py)
            }
        }
        Value::String(v) => v.into_py(py),
        Value::Array(list) => {
            let mut out = Vec::with_capacity(list.len());
            for item in list {
                out.push(json_to_py(py, item)?);
            }
            out.into_py(py)
        }
        Value::Object(map) => {
            let out = PyDict::new_bound(py);
            for (key, value) in map {
                out.set_item(key, json_to_py(py, value)?)?;
            }
            out.into_py(py)
        }
    })
}

fn convert_db_error(error: DbError) -> PyErr {
    match error {
        DbError::MissingTable(_) | DbError::MissingField(_) | DbError::MissingRecord(_) => {
            PyKeyError::new_err(error.to_string())
        }
        DbError::UniqueViolation(_) | DbError::TypeMismatch { .. } => {
            PyValueError::new_err(error.to_string())
        }
        DbError::Io(_) => PyIOError::new_err(error.to_string()),
    }
}

#[pyclass]
#[derive(Clone)]
struct Record {
    #[pyo3(get)]
    id: u64,
    #[pyo3(get)]
    data: PyObject,
}

#[pymethods]
impl Record {
    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        Ok(format!(
            "Record(id={}, data={})",
            self.id,
            self.data.bind(py).repr()?
        ))
    }
}

#[pyclass]
#[derive(Clone)]
struct Query {
    table: String,
    filters: Vec<(String, Value)>,
    order_by: Option<(String, bool)>,
    limit: Option<usize>,
}

#[pymethods]
impl Query {
    #[new]
    fn new(table: String) -> Self {
        Self {
            table,
            filters: Vec::new(),
            order_by: None,
            limit: None,
        }
    }

    fn where_eq(
        mut slf: PyRefMut<'_, Self>,
        field: String,
        value: Bound<'_, PyAny>,
    ) -> PyResult<PyRefMut<'_, Self>> {
        slf.filters.push((field, py_to_json(value)?));
        Ok(slf)
    }

    fn order_by(
        mut slf: PyRefMut<'_, Self>,
        field: String,
        descending: Option<bool>,
    ) -> PyRefMut<'_, Self> {
        slf.order_by = Some((field, descending.unwrap_or(false)));
        slf
    }

    fn take(mut slf: PyRefMut<'_, Self>, count: usize) -> PyRefMut<'_, Self> {
        slf.limit = Some(count);
        slf
    }
}

#[pyclass]
struct Database {
    engine: Engine,
    storage_path: Option<PathBuf>,
    transaction_snapshot: Option<Engine>,
}

#[pymethods]
impl Database {
    #[new]
    #[pyo3(signature = (storage_path=None))]
    fn new(storage_path: Option<String>) -> PyResult<Self> {
        let path = storage_path.map(PathBuf::from);
        let engine = if let Some(path) = &path {
            if path.exists() {
                let raw = fs::read_to_string(path)
                    .map_err(|err| PyIOError::new_err(format!("cannot read db file: {err}")))?;
                serde_json::from_str::<Engine>(&raw)
                    .map_err(|err| PyValueError::new_err(format!("cannot decode db file: {err}")))?
            } else {
                Engine::new()
            }
        } else {
            Engine::new()
        };

        Ok(Self {
            engine,
            storage_path: path,
            transaction_snapshot: None,
        })
    }

    fn create_table(&mut self, name: String, schema: Bound<'_, PyDict>) -> PyResult<()> {
        let mut native_schema = HashMap::new();
        for (field, definition) in schema.iter() {
            let field_name = field.extract::<String>()?;
            let definition = definition.downcast::<PyDict>()?;
            let raw_type = definition
                .get_item("type")?
                .ok_or_else(|| PyValueError::new_err("schema field requires `type`"))?
                .extract::<String>()?;
            let field_type = FieldType::from_str(&raw_type).ok_or_else(|| {
                PyValueError::new_err(format!("unsupported field type `{raw_type}`"))
            })?;
            let required = definition
                .get_item("required")?
                .map(|it| it.extract::<bool>())
                .transpose()?
                .unwrap_or(false);
            let unique = definition
                .get_item("unique")?
                .map(|it| it.extract::<bool>())
                .transpose()?
                .unwrap_or(false);

            native_schema.insert(
                field_name,
                FieldDef {
                    field_type,
                    required,
                    unique,
                },
            );
        }

        self.engine.create_table(&name, native_schema);
        self.persist()?;
        Ok(())
    }

    fn create_index(&mut self, table: String, field: String) -> PyResult<()> {
        let target = self.engine.table_mut(&table).map_err(convert_db_error)?;
        if !target.schema.contains_key(&field) {
            return Err(PyKeyError::new_err(format!(
                "field `{field}` does not exist"
            )));
        }
        if !target.indexes.contains(&field) {
            target.indexes.push(field);
        }
        self.persist()?;
        Ok(())
    }

    fn insert(&mut self, table: String, payload: Bound<'_, PyDict>) -> PyResult<u64> {
        let mut data = Map::new();
        for (key, value) in payload.iter() {
            data.insert(key.extract::<String>()?, py_to_json(value)?);
        }

        let id = self
            .engine
            .table_mut(&table)
            .map_err(convert_db_error)?
            .insert(data)
            .map_err(convert_db_error)?;
        self.persist()?;
        Ok(id)
    }

    fn update(&mut self, table: String, record_id: u64, patch: Bound<'_, PyDict>) -> PyResult<()> {
        let mut patch_data = Map::new();
        for (key, value) in patch.iter() {
            patch_data.insert(key.extract::<String>()?, py_to_json(value)?);
        }

        self.engine
            .table_mut(&table)
            .map_err(convert_db_error)?
            .update(record_id, patch_data)
            .map_err(convert_db_error)?;

        self.persist()?;
        Ok(())
    }

    fn delete(&mut self, table: String, record_id: u64) -> PyResult<()> {
        let removed = self
            .engine
            .table_mut(&table)
            .map_err(convert_db_error)?
            .records
            .remove(&record_id);
        if removed.is_none() {
            return Err(convert_db_error(DbError::MissingRecord(record_id)));
        }
        self.persist()?;
        Ok(())
    }

    fn fetch_all(&self, py: Python<'_>, table: String) -> PyResult<Vec<Record>> {
        let target = self.engine.table(&table).map_err(convert_db_error)?;
        let mut out = Vec::new();
        for (id, data) in &target.records {
            out.push(Record {
                id: *id,
                data: json_to_py(py, &Value::Object(data.clone()))?,
            });
        }
        Ok(out)
    }

    fn query(&self, py: Python<'_>, query: PyRef<'_, Query>) -> PyResult<Vec<Record>> {
        let table = self.engine.table(&query.table).map_err(convert_db_error)?;
        let mut rows: Vec<(u64, Map<String, Value>)> = table
            .records
            .iter()
            .map(|(id, data)| (*id, data.clone()))
            .collect();

        for (field, expected) in &query.filters {
            rows.retain(|(_, row)| {
                row.get(field)
                    .map(|actual| actual == expected)
                    .unwrap_or(false)
            });
        }

        if let Some((field, descending)) = &query.order_by {
            rows.sort_by(|(_, left), (_, right)| {
                let left_value = left.get(field).unwrap_or(&Value::Null);
                let right_value = right.get(field).unwrap_or(&Value::Null);
                let cmp = value_cmp(left_value, right_value);
                if *descending {
                    cmp.reverse()
                } else {
                    cmp
                }
            });
        }

        if let Some(limit) = query.limit {
            rows.truncate(limit);
        }

        let mut output = Vec::with_capacity(rows.len());
        for (id, row) in rows {
            output.push(Record {
                id,
                data: json_to_py(py, &Value::Object(row))?,
            });
        }
        Ok(output)
    }

    fn execute_sql(&mut self, py: Python<'_>, sql: String) -> PyResult<PyObject> {
        let tokens: Vec<&str> = sql.split_whitespace().collect();
        if tokens.is_empty() {
            return Err(PyValueError::new_err("empty statement"));
        }

        match tokens[0].to_ascii_uppercase().as_str() {
            "SHOW" => {
                let tables: Vec<String> = self.engine.tables.keys().cloned().collect();
                Ok(tables.into_py(py))
            }
            "COUNT" => {
                if tokens.len() != 2 {
                    return Err(PyValueError::new_err("COUNT syntax: COUNT <table>"));
                }
                let count = self
                    .engine
                    .table(tokens[1])
                    .map_err(convert_db_error)?
                    .records
                    .len();
                Ok(count.into_py(py))
            }
            _ => Err(PyRuntimeError::new_err(
                "unsupported SQL verb. supported: SHOW, COUNT <table>",
            )),
        }
    }

    fn export_csv(&self, table: String, destination: String) -> PyResult<()> {
        let target = self.engine.table(&table).map_err(convert_db_error)?;
        let mut writer = csv::Writer::from_path(Path::new(&destination))
            .map_err(|err| PyIOError::new_err(err.to_string()))?;

        let mut headers: Vec<String> = target.schema.keys().cloned().collect();
        headers.sort();
        writer
            .write_record(&headers)
            .map_err(|err| PyIOError::new_err(err.to_string()))?;

        for record in target.records.values() {
            let row: Vec<String> = headers
                .iter()
                .map(|field| {
                    record
                        .get(field)
                        .map(|it| it.to_string())
                        .unwrap_or_default()
                })
                .collect();
            writer
                .write_record(row)
                .map_err(|err| PyIOError::new_err(err.to_string()))?;
        }

        writer
            .flush()
            .map_err(|err| PyIOError::new_err(err.to_string()))?;
        Ok(())
    }

    fn begin_transaction(&mut self) {
        if self.transaction_snapshot.is_none() {
            self.transaction_snapshot = Some(self.engine.clone());
        }
    }

    fn rollback(&mut self) {
        if let Some(snapshot) = self.transaction_snapshot.take() {
            self.engine = snapshot;
        }
    }

    fn commit(&mut self) -> PyResult<()> {
        self.transaction_snapshot = None;
        self.persist()?;
        Ok(())
    }

    fn save(&self) -> PyResult<()> {
        self.persist()
    }
}

impl Database {
    fn persist(&self) -> PyResult<()> {
        if let Some(path) = &self.storage_path {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|err| convert_db_error(DbError::Io(err.to_string())))?;
            }

            let payload = serde_json::to_string_pretty(&self.engine)
                .map_err(|err| PyRuntimeError::new_err(err.to_string()))?;
            fs::write(path, payload)
                .map_err(|err| convert_db_error(DbError::Io(err.to_string())))?;
        }
        Ok(())
    }
}

#[pymodule]
fn _core(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Database>()?;
    module.add_class::<Query>()?;
    module.add_class::<Record>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_schema() -> HashMap<String, FieldDef> {
        HashMap::from([
            (
                "name".to_string(),
                FieldDef {
                    field_type: FieldType::String,
                    required: true,
                    unique: false,
                },
            ),
            (
                "email".to_string(),
                FieldDef {
                    field_type: FieldType::String,
                    required: true,
                    unique: true,
                },
            ),
        ])
    }

    #[test]
    fn validates_uniqueness() {
        let mut table = Table::new(sample_schema());
        let mut first = Map::new();
        first.insert("name".to_string(), Value::String("Ana".to_string()));
        first.insert(
            "email".to_string(),
            Value::String("ana@example.com".to_string()),
        );
        table.insert(first).expect("first insert should pass");

        let mut duplicate = Map::new();
        duplicate.insert("name".to_string(), Value::String("Rui".to_string()));
        duplicate.insert(
            "email".to_string(),
            Value::String("ana@example.com".to_string()),
        );

        let error = table.insert(duplicate).expect_err("duplicate should fail");
        assert!(matches!(error, DbError::UniqueViolation(_)));
    }
}
