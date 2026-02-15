use pyo3::exceptions::{PyIOError, PyKeyError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use rusqlite::types::{Value as SqlValue, ValueRef};
use rusqlite::Connection;
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
    #[error("table `{0}` already exists")]
    TableExists(String),
    #[error("field `{0}` is not part of the schema")]
    UnknownField(String),
    #[error("invalid identifier `{0}`")]
    InvalidIdentifier(String),
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

    fn sql_label(&self) -> &'static str {
        match self {
            Self::String => "TEXT",
            Self::Integer => "INTEGER",
            Self::Float => "REAL",
            Self::Boolean => "INTEGER",
            Self::Json => "TEXT",
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
        for field in payload.keys() {
            if !self.schema.contains_key(field) {
                return Err(DbError::UnknownField(field.clone()));
            }
        }

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

    fn create_table(&mut self, name: &str, schema: HashMap<String, FieldDef>) -> DbResult<()> {
        if self.tables.contains_key(name) {
            return Err(DbError::TableExists(name.to_string()));
        }
        self.tables.insert(name.to_string(), Table::new(schema));
        Ok(())
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

fn validate_identifier(identifier: &str) -> DbResult<()> {
    if identifier.is_empty()
        || !identifier
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
    {
        return Err(DbError::InvalidIdentifier(identifier.to_string()));
    }
    Ok(())
}

fn json_to_sql(value: &Value) -> SqlValue {
    match value {
        Value::Null => SqlValue::Null,
        Value::Bool(flag) => SqlValue::Integer(i64::from(*flag)),
        Value::Number(number) => {
            if let Some(int) = number.as_i64() {
                SqlValue::Integer(int)
            } else if let Some(uint) = number.as_u64() {
                SqlValue::Integer(uint as i64)
            } else {
                SqlValue::Real(number.as_f64().unwrap_or_default())
            }
        }
        Value::String(text) => SqlValue::Text(text.clone()),
        Value::Array(_) | Value::Object(_) => SqlValue::Text(value.to_string()),
    }
}

fn sql_ref_to_json(value: ValueRef<'_>) -> Value {
    match value {
        ValueRef::Null => Value::Null,
        ValueRef::Integer(int) => Value::Number(int.into()),
        ValueRef::Real(float) => serde_json::Number::from_f64(float)
            .map(Value::Number)
            .unwrap_or(Value::Null),
        ValueRef::Text(bytes) => {
            let text = String::from_utf8_lossy(bytes).to_string();
            serde_json::from_str::<Value>(&text).unwrap_or(Value::String(text))
        }
        ValueRef::Blob(bytes) => {
            let hex = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
            Value::String(hex)
        }
    }
}

fn convert_db_error(error: DbError) -> PyErr {
    match error {
        DbError::MissingTable(_) | DbError::MissingField(_) | DbError::MissingRecord(_) => {
            PyKeyError::new_err(error.to_string())
        }
        DbError::UniqueViolation(_)
        | DbError::TypeMismatch { .. }
        | DbError::TableExists(_)
        | DbError::UnknownField(_)
        | DbError::InvalidIdentifier(_) => PyValueError::new_err(error.to_string()),
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
        validate_identifier(&name).map_err(convert_db_error)?;
        let mut native_schema = HashMap::new();
        for (field, definition) in schema.iter() {
            let field_name = field.extract::<String>()?;
            validate_identifier(&field_name).map_err(convert_db_error)?;
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

        self.engine
            .create_table(&name, native_schema)
            .map_err(convert_db_error)?;
        self.persist()?;
        Ok(())
    }

    fn create_index(&mut self, table: String, field: String) -> PyResult<()> {
        validate_identifier(&table).map_err(convert_db_error)?;
        validate_identifier(&field).map_err(convert_db_error)?;
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
        validate_identifier(&table).map_err(convert_db_error)?;
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
        validate_identifier(&table).map_err(convert_db_error)?;
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
        validate_identifier(&table).map_err(convert_db_error)?;
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
        validate_identifier(&table).map_err(convert_db_error)?;
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
        validate_identifier(&query.table).map_err(convert_db_error)?;
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
        validate_identifier(&table).map_err(convert_db_error)?;
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

    fn export_jsonl(&self, table: String, destination: String) -> PyResult<()> {
        validate_identifier(&table).map_err(convert_db_error)?;
        let target = self.engine.table(&table).map_err(convert_db_error)?;
        let mut rows: Vec<(u64, &Map<String, Value>)> =
            target.records.iter().map(|(id, row)| (*id, row)).collect();
        rows.sort_by_key(|(id, _)| *id);

        let mut output = String::new();
        for (id, row) in rows {
            let mut record = Map::new();
            record.insert("id".to_string(), Value::Number(id.into()));
            for (key, value) in row {
                record.insert(key.clone(), value.clone());
            }
            output.push_str(
                &serde_json::to_string(&Value::Object(record))
                    .map_err(|err| PyRuntimeError::new_err(err.to_string()))?,
            );
            output.push('\n');
        }

        fs::write(destination, output).map_err(|err| PyIOError::new_err(err.to_string()))?;
        Ok(())
    }

    fn import_jsonl(&mut self, table: String, source: String) -> PyResult<usize> {
        validate_identifier(&table).map_err(convert_db_error)?;
        let raw = fs::read_to_string(source).map_err(|err| PyIOError::new_err(err.to_string()))?;
        let target = self.engine.table_mut(&table).map_err(convert_db_error)?;
        let mut imported = 0usize;

        for (line_number, line) in raw.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            let value: Value = serde_json::from_str(line).map_err(|err| {
                PyValueError::new_err(format!("invalid JSONL at line {}: {err}", line_number + 1))
            })?;
            let mut payload = value
                .as_object()
                .cloned()
                .ok_or_else(|| PyValueError::new_err("JSONL row must be an object"))?;
            payload.remove("id");

            target.insert(payload).map_err(convert_db_error)?;
            imported += 1;
        }

        self.persist()?;
        Ok(imported)
    }

    #[pyo3(signature = (table, source, source_table=None))]
    fn import_sqlite(
        &mut self,
        table: String,
        source: String,
        source_table: Option<String>,
    ) -> PyResult<usize> {
        validate_identifier(&table).map_err(convert_db_error)?;
        if let Some(source_name) = &source_table {
            validate_identifier(source_name).map_err(convert_db_error)?;
        }

        let source_name = source_table.unwrap_or_else(|| table.clone());
        let conn = Connection::open(source).map_err(|err| PyIOError::new_err(err.to_string()))?;
        let target = self.engine.table_mut(&table).map_err(convert_db_error)?;

        let mut statement = conn
            .prepare(&format!("SELECT * FROM \"{source_name}\""))
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        let column_names: Vec<String> = statement
            .column_names()
            .into_iter()
            .map(ToString::to_string)
            .collect();

        let mut rows = statement
            .query([])
            .map_err(|err| PyValueError::new_err(err.to_string()))?;
        let mut imported = 0usize;

        while let Some(row) = rows
            .next()
            .map_err(|err| PyValueError::new_err(err.to_string()))?
        {
            let mut payload = Map::new();
            for (index, name) in column_names.iter().enumerate() {
                if name == "id" {
                    continue;
                }
                if !target.schema.contains_key(name) {
                    continue;
                }
                let value_ref = row
                    .get_ref(index)
                    .map_err(|err| PyValueError::new_err(err.to_string()))?;
                payload.insert(name.clone(), sql_ref_to_json(value_ref));
            }
            target.insert(payload).map_err(convert_db_error)?;
            imported += 1;
        }

        self.persist()?;
        Ok(imported)
    }

    fn export_sqlite(&self, table: String, destination: String) -> PyResult<()> {
        validate_identifier(&table).map_err(convert_db_error)?;
        let target = self.engine.table(&table).map_err(convert_db_error)?;
        let mut conn =
            Connection::open(destination).map_err(|err| PyIOError::new_err(err.to_string()))?;

        let mut fields: Vec<(String, FieldDef)> = target
            .schema
            .iter()
            .map(|(name, def)| (name.clone(), def.clone()))
            .collect();
        fields.sort_by(|left, right| left.0.cmp(&right.0));

        let columns = fields
            .iter()
            .map(|(name, def)| format!("\"{name}\" {}", def.field_type.sql_label()))
            .collect::<Vec<_>>()
            .join(", ");
        conn.execute(
            &format!("CREATE TABLE IF NOT EXISTS \"{table}\" (id INTEGER PRIMARY KEY, {columns})"),
            [],
        )
        .map_err(|err| PyIOError::new_err(err.to_string()))?;
        conn.execute(&format!("DELETE FROM \"{table}\""), [])
            .map_err(|err| PyIOError::new_err(err.to_string()))?;

        let field_names: Vec<String> = fields.into_iter().map(|(name, _)| name).collect();
        let quoted_fields = field_names
            .iter()
            .map(|name| format!("\"{name}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let placeholders = (0..=field_names.len())
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");
        let statement =
            format!("INSERT INTO \"{table}\" (id, {quoted_fields}) VALUES ({placeholders})");

        let mut rows: Vec<(u64, &Map<String, Value>)> =
            target.records.iter().map(|(id, row)| (*id, row)).collect();
        rows.sort_by_key(|(id, _)| *id);

        let transaction = conn
            .transaction()
            .map_err(|err| PyIOError::new_err(err.to_string()))?;
        {
            let mut insert = transaction
                .prepare(&statement)
                .map_err(|err| PyIOError::new_err(err.to_string()))?;
            for (id, row) in rows {
                let mut params = Vec::with_capacity(field_names.len() + 1);
                params.push(SqlValue::Integer(id as i64));
                for field in &field_names {
                    let value = row.get(field).unwrap_or(&Value::Null);
                    params.push(json_to_sql(value));
                }
                insert
                    .execute(rusqlite::params_from_iter(params))
                    .map_err(|err| PyIOError::new_err(err.to_string()))?;
            }
        }
        transaction
            .commit()
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
    use tempfile::NamedTempFile;

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

    #[test]
    fn rejects_unknown_fields() {
        let mut table = Table::new(sample_schema());
        let mut payload = Map::new();
        payload.insert("name".to_string(), Value::String("Ana".to_string()));
        payload.insert(
            "email".to_string(),
            Value::String("ana@example.com".to_string()),
        );
        payload.insert("role".to_string(), Value::String("admin".to_string()));

        let error = table
            .insert(payload)
            .expect_err("unknown field should fail");
        assert!(matches!(error, DbError::UnknownField(_)));
    }

    #[test]
    fn exports_and_imports_sqlite() {
        let mut source_engine = Engine::new();
        source_engine
            .create_table("users", sample_schema())
            .expect("table creation should work");
        let users = source_engine
            .table_mut("users")
            .expect("users table should exist");

        let mut payload = Map::new();
        payload.insert("name".to_string(), Value::String("Ana".to_string()));
        payload.insert(
            "email".to_string(),
            Value::String("ana@example.com".to_string()),
        );
        users.insert(payload).expect("insert should succeed");

        let database = Database {
            engine: source_engine,
            storage_path: None,
            transaction_snapshot: None,
        };

        let sqlite_file = NamedTempFile::new().expect("sqlite temp file");
        database
            .export_sqlite(
                "users".to_string(),
                sqlite_file.path().to_string_lossy().to_string(),
            )
            .expect("sqlite export should succeed");

        let mut target_engine = Engine::new();
        target_engine
            .create_table("users", sample_schema())
            .expect("table creation should work");
        let mut target_db = Database {
            engine: target_engine,
            storage_path: None,
            transaction_snapshot: None,
        };

        let imported = target_db
            .import_sqlite(
                "users".to_string(),
                sqlite_file.path().to_string_lossy().to_string(),
                None,
            )
            .expect("import should work");
        assert_eq!(imported, 1);
        assert_eq!(
            target_db
                .engine
                .table("users")
                .expect("users table")
                .records
                .len(),
            1
        );
    }
}
