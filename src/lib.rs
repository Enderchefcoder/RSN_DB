use pyo3::exceptions::{PyKeyError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum FieldType {
    String,
    Int,
    Float,
    Bool,
    Doc,
    Array,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Doc(JsonValue),
    Array(Vec<Value>),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Row {
    id: String,
    fields: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Table {
    name: String,
    schema: HashMap<String, FieldType>,
    flexible: bool,
    id_prefix: String,
    next_id: usize,
    rows: Vec<Row>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Edge {
    from_table: String,
    from_id: String,
    label: String,
    to_table: String,
    to_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CoreState {
    tables: HashMap<String, Table>,
    edges: Vec<Edge>,
    kv_store: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistentState {
    core: CoreState,
    checkpoints: HashMap<String, CoreState>,
}

#[derive(Debug)]
struct Engine {
    core: CoreState,
    checkpoints: HashMap<String, CoreState>,
    undo_stack: Vec<CoreState>,
    redo_stack: Vec<CoreState>,
}

#[derive(thiserror::Error, Debug)]
enum EngineError {
    #[error("table '{0}' does not exist")]
    MissingTable(String),
    #[error("row '{row_id}' not found in table '{table}'")]
    MissingRow { table: String, row_id: String },
    #[error("schema validation failed for field '{field}': expected {expected:?}, got {actual:?}")]
    TypeMismatch {
        field: String,
        expected: FieldType,
        actual: Value,
    },
    #[error("invalid operation: {0}")]
    InvalidOperation(String),
}

#[derive(Debug, Clone)]
struct Condition {
    field: String,
    operator: String,
    value: Value,
}

impl Engine {
    fn new() -> Self {
        Self {
            core: CoreState {
                tables: HashMap::new(),
                edges: Vec::new(),
                kv_store: HashMap::new(),
            },
            checkpoints: HashMap::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    fn snapshot_mutation(&mut self) {
        self.undo_stack.push(self.core.clone());
        self.redo_stack.clear();
    }

    fn create_table(
        &mut self,
        name: &str,
        schema: HashMap<String, FieldType>,
        flexible: bool,
    ) -> Result<(), EngineError> {
        self.snapshot_mutation();
        let lower_name = name.to_ascii_lowercase();
        let prefix = lower_name.chars().take(3).collect::<String>();
        self.core.tables.insert(
            lower_name.clone(),
            Table {
                name: lower_name,
                schema,
                flexible,
                id_prefix: if prefix.is_empty() {
                    "row".to_string()
                } else {
                    prefix
                },
                next_id: 1,
                rows: Vec::new(),
            },
        );
        Ok(())
    }

    fn insert_row(
        &mut self,
        table_name: &str,
        row_data: HashMap<String, Value>,
    ) -> Result<String, EngineError> {
        self.snapshot_mutation();
        let table = self
            .core
            .tables
            .get_mut(&table_name.to_ascii_lowercase())
            .ok_or_else(|| EngineError::MissingTable(table_name.to_string()))?;

        for (field, value) in &row_data {
            if let Some(expected_type) = table.schema.get(field) {
                if !value.matches(expected_type) {
                    return Err(EngineError::TypeMismatch {
                        field: field.clone(),
                        expected: expected_type.clone(),
                        actual: value.clone(),
                    });
                }
            } else if !table.flexible {
                return Err(EngineError::InvalidOperation(format!(
                    "field '{field}' is not part of table schema"
                )));
            }
        }

        let row_id = format!("{}_{:05}", table.id_prefix, table.next_id);
        table.next_id += 1;
        table.rows.push(Row {
            id: row_id.clone(),
            fields: row_data,
        });
        Ok(row_id)
    }

    fn read_rows(
        &self,
        table_name: &str,
        condition: Option<Condition>,
    ) -> Result<Vec<Row>, EngineError> {
        let table = self
            .core
            .tables
            .get(&table_name.to_ascii_lowercase())
            .ok_or_else(|| EngineError::MissingTable(table_name.to_string()))?;

        let rows = table
            .rows
            .iter()
            .filter(|row| {
                condition
                    .as_ref()
                    .map(|cond| cond.matches(row))
                    .unwrap_or(true)
            })
            .cloned()
            .collect();
        Ok(rows)
    }

    fn update_rows(
        &mut self,
        table_name: &str,
        condition: Condition,
        updates: HashMap<String, Value>,
    ) -> Result<usize, EngineError> {
        self.snapshot_mutation();
        let table = self
            .core
            .tables
            .get_mut(&table_name.to_ascii_lowercase())
            .ok_or_else(|| EngineError::MissingTable(table_name.to_string()))?;

        for (field, value) in &updates {
            if let Some(expected_type) = table.schema.get(field) {
                if !value.matches(expected_type) {
                    return Err(EngineError::TypeMismatch {
                        field: field.clone(),
                        expected: expected_type.clone(),
                        actual: value.clone(),
                    });
                }
            }
        }

        let mut count = 0;
        for row in &mut table.rows {
            if condition.matches(row) {
                for (field, value) in &updates {
                    row.fields.insert(field.clone(), value.clone());
                }
                count += 1;
            }
        }
        Ok(count)
    }

    fn remove_rows(
        &mut self,
        table_name: &str,
        condition: Condition,
    ) -> Result<usize, EngineError> {
        self.snapshot_mutation();
        let table = self
            .core
            .tables
            .get_mut(&table_name.to_ascii_lowercase())
            .ok_or_else(|| EngineError::MissingTable(table_name.to_string()))?;

        let before = table.rows.len();
        let removed_ids: Vec<String> = table
            .rows
            .iter()
            .filter(|row| condition.matches(row))
            .map(|row| row.id.clone())
            .collect();

        table.rows.retain(|row| !removed_ids.contains(&row.id));
        self.core.edges.retain(|edge| {
            !(edge.from_table == table.name && removed_ids.contains(&edge.from_id)
                || edge.to_table == table.name && removed_ids.contains(&edge.to_id))
        });

        Ok(before.saturating_sub(table.rows.len()))
    }

    fn link(
        &mut self,
        from_table: &str,
        from_id: &str,
        label: &str,
        to_table: &str,
        to_id: &str,
    ) -> Result<(), EngineError> {
        self.snapshot_mutation();
        self.ensure_row_exists(from_table, from_id)?;
        self.ensure_row_exists(to_table, to_id)?;
        self.core.edges.push(Edge {
            from_table: from_table.to_ascii_lowercase(),
            from_id: from_id.to_string(),
            label: label.to_ascii_uppercase(),
            to_table: to_table.to_ascii_lowercase(),
            to_id: to_id.to_string(),
        });
        Ok(())
    }

    fn unlink(
        &mut self,
        from_table: &str,
        from_id: &str,
        label: &str,
        to_table: &str,
        to_id: &str,
    ) -> usize {
        self.snapshot_mutation();
        let before = self.core.edges.len();
        self.core.edges.retain(|edge| {
            !(edge.from_table == from_table.to_ascii_lowercase()
                && edge.from_id == from_id
                && edge.label == label.to_ascii_uppercase()
                && edge.to_table == to_table.to_ascii_lowercase()
                && edge.to_id == to_id)
        });
        before.saturating_sub(self.core.edges.len())
    }

    fn walk(
        &self,
        from_table: &str,
        from_id: &str,
        label: &str,
    ) -> Result<Vec<(String, String)>, EngineError> {
        self.ensure_row_exists(from_table, from_id)?;
        let mut queue = VecDeque::from([(from_table.to_ascii_lowercase(), from_id.to_string())]);
        let mut visited = HashMap::new();
        visited.insert((from_table.to_ascii_lowercase(), from_id.to_string()), ());
        let mut results = Vec::new();

        while let Some((current_table, current_id)) = queue.pop_front() {
            for edge in self.core.edges.iter().filter(|candidate| {
                candidate.from_table == current_table
                    && candidate.from_id == current_id
                    && candidate.label == label.to_ascii_uppercase()
            }) {
                let key = (edge.to_table.clone(), edge.to_id.clone());
                if !visited.contains_key(&key) {
                    visited.insert(key.clone(), ());
                    queue.push_back(key.clone());
                    results.push(key);
                }
            }
        }

        Ok(results)
    }

    fn put(&mut self, key: &str, value: Value) {
        self.snapshot_mutation();
        self.core.kv_store.insert(key.to_string(), value);
    }

    fn get(&self, key: &str) -> Option<Value> {
        self.core.kv_store.get(key).cloned()
    }

    fn drop_key(&mut self, key: &str) -> bool {
        self.snapshot_mutation();
        self.core.kv_store.remove(key).is_some()
    }

    fn checkpoint(&mut self, name: &str) {
        self.checkpoints.insert(name.to_string(), self.core.clone());
    }

    fn rollback_to(&mut self, name: &str) -> Result<(), EngineError> {
        let checkpoint = self.checkpoints.get(name).cloned().ok_or_else(|| {
            EngineError::InvalidOperation(format!("checkpoint '{name}' not found"))
        })?;
        self.snapshot_mutation();
        self.core = checkpoint;
        Ok(())
    }

    fn undo(&mut self) -> bool {
        if let Some(previous) = self.undo_stack.pop() {
            self.redo_stack.push(self.core.clone());
            self.core = previous;
            return true;
        }
        false
    }

    fn redo(&mut self) -> bool {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.core.clone());
            self.core = next;
            return true;
        }
        false
    }

    fn save_to_path(&self, path: &str) -> Result<(), EngineError> {
        let payload = PersistentState {
            core: self.core.clone(),
            checkpoints: self.checkpoints.clone(),
        };
        let json = serde_json::to_string_pretty(&payload)
            .map_err(|error| EngineError::InvalidOperation(error.to_string()))?;
        fs::write(path, json).map_err(|error| EngineError::InvalidOperation(error.to_string()))?;
        Ok(())
    }

    fn load_from_path(&mut self, path: &str) -> Result<(), EngineError> {
        let data = fs::read_to_string(path)
            .map_err(|error| EngineError::InvalidOperation(error.to_string()))?;
        let payload: PersistentState = serde_json::from_str(&data)
            .map_err(|error| EngineError::InvalidOperation(error.to_string()))?;
        self.core = payload.core;
        self.checkpoints = payload.checkpoints;
        self.undo_stack.clear();
        self.redo_stack.clear();
        Ok(())
    }

    fn ensure_row_exists(&self, table_name: &str, row_id: &str) -> Result<(), EngineError> {
        let table_key = table_name.to_ascii_lowercase();
        let table = self
            .core
            .tables
            .get(&table_key)
            .ok_or_else(|| EngineError::MissingTable(table_name.to_string()))?;
        if table.rows.iter().any(|row| row.id == row_id) {
            Ok(())
        } else {
            Err(EngineError::MissingRow {
                table: table_key,
                row_id: row_id.to_string(),
            })
        }
    }
}

impl Value {
    fn matches(&self, field_type: &FieldType) -> bool {
        matches!(
            (self, field_type),
            (Value::String(_), FieldType::String)
                | (Value::Int(_), FieldType::Int)
                | (Value::Float(_), FieldType::Float)
                | (Value::Bool(_), FieldType::Bool)
                | (Value::Doc(_), FieldType::Doc)
                | (Value::Array(_), FieldType::Array)
                | (Value::Null, _)
        )
    }

    fn compare(&self, operator: &str, rhs: &Value) -> bool {
        match operator {
            "=" | "==" => self == rhs,
            "!=" => self != rhs,
            ">" => compare_numeric(self, rhs)
                .map(|(l, r)| l > r)
                .unwrap_or(false),
            "<" => compare_numeric(self, rhs)
                .map(|(l, r)| l < r)
                .unwrap_or(false),
            ">=" => compare_numeric(self, rhs)
                .map(|(l, r)| l >= r)
                .unwrap_or(false),
            "<=" => compare_numeric(self, rhs)
                .map(|(l, r)| l <= r)
                .unwrap_or(false),
            "contains" => match (self, rhs) {
                (Value::String(left), Value::String(right)) => left.contains(right),
                (Value::Array(items), candidate) => items.contains(candidate),
                _ => false,
            },
            _ => false,
        }
    }
}

impl Condition {
    fn matches(&self, row: &Row) -> bool {
        row.fields
            .get(&self.field)
            .map(|actual| actual.compare(&self.operator, &self.value))
            .unwrap_or(false)
    }
}

fn compare_numeric(lhs: &Value, rhs: &Value) -> Option<(f64, f64)> {
    let left = match lhs {
        Value::Int(value) => *value as f64,
        Value::Float(value) => *value,
        _ => return None,
    };
    let right = match rhs {
        Value::Int(value) => *value as f64,
        Value::Float(value) => *value,
        _ => return None,
    };
    Some((left, right))
}

fn parse_field_type(value: &str) -> Option<FieldType> {
    match value.to_ascii_lowercase().as_str() {
        "string" => Some(FieldType::String),
        "int" => Some(FieldType::Int),
        "float" => Some(FieldType::Float),
        "bool" => Some(FieldType::Bool),
        "doc" => Some(FieldType::Doc),
        "array" => Some(FieldType::Array),
        _ => None,
    }
}

fn to_py_err(error: EngineError) -> PyErr {
    match error {
        EngineError::MissingTable(_) | EngineError::MissingRow { .. } => {
            PyKeyError::new_err(error.to_string())
        }
        EngineError::TypeMismatch { .. } => PyValueError::new_err(error.to_string()),
        EngineError::InvalidOperation(_) => PyRuntimeError::new_err(error.to_string()),
    }
}

fn py_to_value(input: &Bound<'_, PyAny>) -> PyResult<Value> {
    if input.is_none() {
        return Ok(Value::Null);
    }
    if let Ok(value) = input.extract::<bool>() {
        return Ok(Value::Bool(value));
    }
    if let Ok(value) = input.extract::<i64>() {
        return Ok(Value::Int(value));
    }
    if let Ok(value) = input.extract::<f64>() {
        return Ok(Value::Float(value));
    }
    if let Ok(value) = input.extract::<String>() {
        return Ok(Value::String(value));
    }
    if let Ok(dict) = input.cast_as::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (key, value) in dict.iter() {
            let key_str = key.extract::<String>()?;
            let json = py_any_to_json(&value)?;
            map.insert(key_str, json);
        }
        return Ok(Value::Doc(JsonValue::Object(map)));
    }
    if let Ok(list) = input.cast_as::<PyList>() {
        let mut values = Vec::with_capacity(list.len());
        for item in list.iter() {
            values.push(py_to_value(&item)?);
        }
        return Ok(Value::Array(values));
    }

    Err(PyValueError::new_err("unsupported python value type"))
}

fn py_any_to_json(input: &Bound<'_, PyAny>) -> PyResult<JsonValue> {
    if input.is_none() {
        return Ok(JsonValue::Null);
    }
    if let Ok(value) = input.extract::<bool>() {
        return Ok(JsonValue::Bool(value));
    }
    if let Ok(value) = input.extract::<i64>() {
        return Ok(JsonValue::Number(value.into()));
    }
    if let Ok(value) = input.extract::<f64>() {
        return serde_json::Number::from_f64(value)
            .map(JsonValue::Number)
            .ok_or_else(|| PyValueError::new_err("float value cannot be represented as JSON"));
    }
    if let Ok(value) = input.extract::<String>() {
        return Ok(JsonValue::String(value));
    }
    if let Ok(dict) = input.cast_as::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (key, value) in dict.iter() {
            map.insert(key.extract::<String>()?, py_any_to_json(&value)?);
        }
        return Ok(JsonValue::Object(map));
    }
    if let Ok(list) = input.cast_as::<PyList>() {
        let mut values = Vec::with_capacity(list.len());
        for item in list.iter() {
            values.push(py_any_to_json(&item)?);
        }
        return Ok(JsonValue::Array(values));
    }

    Err(PyValueError::new_err(
        "unsupported python value for JSON document",
    ))
}

fn value_to_py(value: &Value, py: Python<'_>) -> PyResult<PyObject> {
    match value {
        Value::String(data) => Ok(data.into_py(py)),
        Value::Int(data) => Ok(data.into_py(py)),
        Value::Float(data) => Ok(data.into_py(py)),
        Value::Bool(data) => Ok(data.into_py(py)),
        Value::Null => Ok(py.None()),
        Value::Doc(data) => {
            let text = serde_json::to_string(data)
                .map_err(|error| PyValueError::new_err(error.to_string()))?;
            let json = py.import("json")?;
            let object = json.call_method1("loads", (text,))?;
            Ok(object.into_py(py))
        }
        Value::Array(items) => {
            let list = PyList::empty_bound(py);
            for item in items {
                list.append(value_to_py(item, py)?)?;
            }
            Ok(list.into_py(py))
        }
    }
}

#[pyclass]
struct RSNDatabase {
    engine: Engine,
}

#[pymethods]
impl RSNDatabase {
    #[new]
    fn new() -> Self {
        Self {
            engine: Engine::new(),
        }
    }

    #[pyo3(signature = (name, schema, flexible=true))]
    fn create_table(
        &mut self,
        name: &str,
        schema: HashMap<String, String>,
        flexible: bool,
    ) -> PyResult<()> {
        let mut typed_schema = HashMap::new();
        for (field, value) in schema {
            let field_type = parse_field_type(&value).ok_or_else(|| {
                PyValueError::new_err(format!("unsupported type '{value}' for field '{field}'"))
            })?;
            typed_schema.insert(field, field_type);
        }
        self.engine
            .create_table(name, typed_schema, flexible)
            .map_err(to_py_err)
    }

    fn insert(
        &mut self,
        table: &str,
        row: HashMap<String, PyObject>,
        py: Python<'_>,
    ) -> PyResult<String> {
        let mut parsed = HashMap::new();
        for (field, value) in row {
            parsed.insert(field, py_to_value(&value.bind(py))?);
        }
        self.engine.insert_row(table, parsed).map_err(to_py_err)
    }

    #[pyo3(signature = (table, condition=None))]
    fn read(
        &self,
        table: &str,
        condition: Option<(String, String, PyObject)>,
        py: Python<'_>,
    ) -> PyResult<Vec<PyObject>> {
        let cond = if let Some((field, operator, value)) = condition {
            Some(Condition {
                field,
                operator,
                value: py_to_value(&value.bind(py))?,
            })
        } else {
            None
        };

        let rows = self.engine.read_rows(table, cond).map_err(to_py_err)?;
        let mut output = Vec::with_capacity(rows.len());
        for row in rows {
            let dict = PyDict::new_bound(py);
            dict.set_item("id", row.id)?;
            for (field, value) in row.fields {
                dict.set_item(field, value_to_py(&value, py)?)?;
            }
            output.push(dict.into_py(py));
        }
        Ok(output)
    }

    fn update(
        &mut self,
        table: &str,
        condition: (String, String, PyObject),
        updates: HashMap<String, PyObject>,
        py: Python<'_>,
    ) -> PyResult<usize> {
        let condition = Condition {
            field: condition.0,
            operator: condition.1,
            value: py_to_value(&condition.2.bind(py))?,
        };

        let mut parsed_updates = HashMap::new();
        for (field, value) in updates {
            parsed_updates.insert(field, py_to_value(&value.bind(py))?);
        }

        self.engine
            .update_rows(table, condition, parsed_updates)
            .map_err(to_py_err)
    }

    fn remove(
        &mut self,
        table: &str,
        condition: (String, String, PyObject),
        py: Python<'_>,
    ) -> PyResult<usize> {
        let condition = Condition {
            field: condition.0,
            operator: condition.1,
            value: py_to_value(&condition.2.bind(py))?,
        };
        self.engine.remove_rows(table, condition).map_err(to_py_err)
    }

    fn link(
        &mut self,
        from_table: &str,
        from_id: &str,
        label: &str,
        to_table: &str,
        to_id: &str,
    ) -> PyResult<()> {
        self.engine
            .link(from_table, from_id, label, to_table, to_id)
            .map_err(to_py_err)
    }

    fn unlink(
        &mut self,
        from_table: &str,
        from_id: &str,
        label: &str,
        to_table: &str,
        to_id: &str,
    ) -> PyResult<usize> {
        Ok(self
            .engine
            .unlink(from_table, from_id, label, to_table, to_id))
    }

    fn walk(
        &self,
        from_table: &str,
        from_id: &str,
        label: &str,
    ) -> PyResult<Vec<(String, String)>> {
        self.engine
            .walk(from_table, from_id, label)
            .map_err(to_py_err)
    }

    fn put(&mut self, key: &str, value: PyObject, py: Python<'_>) -> PyResult<()> {
        self.engine.put(key, py_to_value(&value.bind(py))?);
        Ok(())
    }

    fn get(&self, key: &str, py: Python<'_>) -> PyResult<PyObject> {
        match self.engine.get(key) {
            Some(value) => value_to_py(&value, py),
            None => Ok(py.None()),
        }
    }

    fn drop_key(&mut self, key: &str) -> bool {
        self.engine.drop_key(key)
    }

    fn checkpoint(&mut self, name: &str) {
        self.engine.checkpoint(name);
    }

    fn rollback_to(&mut self, name: &str) -> PyResult<()> {
        self.engine.rollback_to(name).map_err(to_py_err)
    }

    fn undo(&mut self) -> bool {
        self.engine.undo()
    }

    fn redo(&mut self) -> bool {
        self.engine.redo()
    }

    fn save(&self, path: &str) -> PyResult<()> {
        self.engine.save_to_path(path).map_err(to_py_err)
    }

    fn load(&mut self, path: &str) -> PyResult<()> {
        self.engine.load_from_path(path).map_err(to_py_err)
    }

    fn tables(&self) -> Vec<(String, usize, bool)> {
        self.engine
            .core
            .tables
            .values()
            .map(|table| (table.name.clone(), table.rows.len(), table.flexible))
            .collect()
    }

    fn describe(&self, table: &str) -> PyResult<HashMap<String, String>> {
        let table = self
            .engine
            .core
            .tables
            .get(&table.to_ascii_lowercase())
            .ok_or_else(|| PyKeyError::new_err(format!("table '{}' does not exist", table)))?;
        Ok(table
            .schema
            .iter()
            .map(|(field, field_type)| (field.clone(), format!("{field_type:?}")))
            .collect())
    }
}

#[pymodule]
fn rsn_db(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<RSNDatabase>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_insert_read_and_walk() {
        let mut engine = Engine::new();

        let mut user_schema = HashMap::new();
        user_schema.insert("name".to_string(), FieldType::String);
        user_schema.insert("age".to_string(), FieldType::Int);
        engine
            .create_table("users", user_schema, true)
            .expect("table should be created");

        let mut post_schema = HashMap::new();
        post_schema.insert("title".to_string(), FieldType::String);
        engine
            .create_table("posts", post_schema, true)
            .expect("table should be created");

        let alice_id = engine
            .insert_row(
                "users",
                HashMap::from([
                    ("name".to_string(), Value::String("Alice".to_string())),
                    ("age".to_string(), Value::Int(30)),
                ]),
            )
            .expect("insert should work");

        let post_id = engine
            .insert_row(
                "posts",
                HashMap::from([("title".to_string(), Value::String("Hello".to_string()))]),
            )
            .expect("insert should work");

        engine
            .link("users", &alice_id, "WROTE", "posts", &post_id)
            .expect("link should work");

        let walked = engine
            .walk("users", &alice_id, "wrote")
            .expect("walk should work");
        assert_eq!(walked, vec![("posts".to_string(), post_id)]);
    }
}
