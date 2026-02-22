pub mod personality;
pub mod graph_rag;

const MAX_RECURSION_DEPTH: usize = 64;

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use personality::{Mode, Personality};
use graph_rag::GraphRagEngine;
use pyo3::exceptions::{PyIOError, PyKeyError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use rand::{thread_rng, Rng};
use rusqlite::types::{Value as SqlValue, ValueRef};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;
use zstd::stream::{decode_all, encode_all};
use lz4_flex::{compress_prepend_size, decompress_size_prepended};

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
    #[error("table `{0}` already exists")]
    TableExists(String),
    #[error("field `{0}` is not part of the schema")]
    UnknownField(String),
    #[error("invalid identifier `{0}`")]
    InvalidIdentifier(String),
}

type DbResult<T> = Result<T, DbError>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
enum CompressionAlgo {
    Zstd,
    Lz4,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FieldDef {
    field_type: FieldType,
    required: bool,
    unique: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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
    fn coerce(&self, value: Value) -> Option<Value> {
        if self.matches(&value) {
            return Some(value);
        }
        match (self, value) {
            (Self::Integer, Value::String(s)) => {
                s.parse::<i64>().ok().map(|i| Value::Number(i.into()))
            }
            (Self::Float, Value::String(s)) => s
                .parse::<f64>()
                .ok()
                .and_then(|f| serde_json::Number::from_f64(f))
                .map(Value::Number),
            (Self::Boolean, Value::String(s)) => match s.to_lowercase().as_str() {
                "true" | "1" | "yes" => Some(Value::Bool(true)),
                "false" | "0" | "no" => Some(Value::Bool(false)),
                _ => None,
            },
            (Self::String, v) => Some(Value::String(v.to_string())),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Table {
    schema: HashMap<String, FieldDef>,
    records: HashMap<u64, Map<String, Value>>,
    next_id: u64,
    #[serde(skip)]
    unique_cache: HashMap<String, HashSet<String>>,
}

impl Table {
    fn new(schema: HashMap<String, FieldDef>) -> Self {
        Self {
            schema,
            records: HashMap::new(),
            next_id: 1,
            unique_cache: HashMap::new(),
        }
    }
    fn validate_payload(
        &self,
        payload: &mut Map<String, Value>,
        updating: Option<u64>,
    ) -> DbResult<()> {
        for field in payload.keys() {
            if !self.schema.contains_key(field) {
                return Err(DbError::UnknownField(field.clone()));
            }
        }
        for (field, def) in &self.schema {
            if let Some(value) = payload.get_mut(field) {
                if !value.is_null() && !def.field_type.matches(value) {
                    if let Some(coerced) = def.field_type.coerce(value.clone()) {
                        *value = coerced;
                    } else {
                        return Err(DbError::TypeMismatch {
                            field: field.clone(),
                            expected: def.field_type.label().to_string(),
                        });
                    }
                }
            } else if def.required {
                return Err(DbError::MissingField(field.clone()));
            }
            if def.unique {
                if let Some(candidate) = payload.get(field) {
                    let serialized = candidate.to_string();
                    if let Some(set) = self.unique_cache.get(field) {
                        if set.contains(&serialized) {
                            if let Some(rid) = updating {
                                if let Some(old_record) = self.records.get(&rid) {
                                    if let Some(old_val) = old_record.get(field) {
                                        if old_val == candidate {
                                            continue;
                                        }
                                    }
                                }
                            }
                            return Err(DbError::UniqueViolation(field.clone()));
                        }
                    }
                }
            }
        }
        Ok(())
    }
    fn insert(&mut self, mut payload: Map<String, Value>) -> DbResult<u64> {
        self.validate_payload(&mut payload, None)?;
        for (f, def) in &self.schema {
            if def.unique {
                if let Some(val) = payload.get(f) {
                    self.unique_cache.entry(f.clone()).or_insert_with(HashSet::new).insert(val.to_string());
                }
            }
        }
        let id = self.next_id;
        self.next_id += 1;
        self.records.insert(id, payload);
        Ok(id)
    }
    fn delete(&mut self, rid: u64) -> DbResult<()> {
        let old = self.records.remove(&rid).ok_or(DbError::MissingRecord(rid))?;
        for (f, def) in &self.schema {
            if def.unique {
                if let Some(val) = old.get(f) {
                    if let Some(set) = self.unique_cache.get_mut(f) {
                        set.remove(&val.to_string());
                    }
                }
            }
        }
        Ok(())
    }
    fn update(&mut self, rid: u64, patch: Map<String, Value>) -> DbResult<()> {
        let mut merged = self
            .records
            .get(&rid)
            .cloned()
            .ok_or(DbError::MissingRecord(rid))?;
        for (k, v) in patch {
            merged.insert(k, v);
        }
        self.validate_payload(&mut merged, Some(rid))?;
        for (f, def) in &self.schema {
            if def.unique {
                if let Some(old_record) = self.records.get(&rid) {
                    if let Some(old_val) = old_record.get(f) {
                        if let Some(set) = self.unique_cache.get_mut(f) {
                            set.remove(&old_val.to_string());
                        }
                    }
                }
                if let Some(new_val) = merged.get(f) {
                    self.unique_cache.entry(f.clone()).or_insert_with(HashSet::new).insert(new_val.to_string());
                }
            }
        }
        self.records.insert(rid, merged);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Engine {
    tables: HashMap<String, Table>,
    aliases: HashMap<String, String>,
    graph_rag: GraphRagEngine,
}

impl Engine {
    fn new() -> Self {
        Self {
            tables: HashMap::new(),
            aliases: HashMap::new(),
            graph_rag: GraphRagEngine::new(),
        }
    }
    fn rebuild_cache(&mut self) {
        self.graph_rag.rebuild_tfidf();
        for table in self.tables.values_mut() {
            table.unique_cache.clear();
            for record in table.records.values() {
                for (f, def) in &table.schema {
                    if def.unique {
                        if let Some(val) = record.get(f) {
                            table.unique_cache.entry(f.clone()).or_insert_with(HashSet::new).insert(val.to_string());
                        }
                    }
                }
            }
        }
    }
    fn create_table(&mut self, name: &str, schema: HashMap<String, FieldDef>) -> DbResult<()> {
        if self.tables.contains_key(name) {
            return Err(DbError::TableExists(name.to_string()));
        }
        self.tables.insert(name.to_string(), Table::new(schema));
        Ok(())
    }
}

#[pyclass]
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
    #[pyo3(signature = (field, value))]
    fn where_eq<'a>(
        mut slf: PyRefMut<'a, Self>,
        field: String,
        value: Bound<'a, PyAny>,
    ) -> PyResult<PyRefMut<'a, Self>> {
        slf.filters.push((field, py_to_json(value)?));
        Ok(slf)
    }
    #[pyo3(signature = (field, descending=None))]
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
    encryption_key: Option<[u8; 32]>,
    compression: CompressionAlgo,
    personality: Personality,
    command_history: Vec<String>,
    batch_mode: bool,
    batch_ops: Vec<String>,
}

#[pymethods]
impl Database {
    #[new]
    #[pyo3(signature = (storage_path=None, encryption_key=None, compression="zstd", mode="professional"))]
    fn new(
        storage_path: Option<String>,
        encryption_key: Option<String>,
        compression: &str,
        mode: &str,
    ) -> PyResult<Self> {
        let mut path = storage_path
            .map(|candidate| sanitize_db_path(&candidate))
            .transpose()?;
        if let Some(ref mut p) = path {
            if p.extension().is_none() {
                p.set_extension("rsndb");
            }
        }
        let key = encryption_key.map(|k| {
            let mut hasher = Sha256::new();
            hasher.update(k.as_bytes());
            let mut res = [0u8; 32];
            res.copy_from_slice(&hasher.finalize());
            res
        });
        let mode_enum = match mode.to_lowercase().as_str() {
            "friendly" => Mode::Friendly,
            "snarky" => Mode::Snarky,
            _ => Mode::Professional,
        };
        let comp_algo = match compression.to_lowercase().as_str() {
            "zstd" => CompressionAlgo::Zstd,
            "lz4" => CompressionAlgo::Lz4,
            "none" => CompressionAlgo::None,
            _ => CompressionAlgo::Zstd,
        };
        let mut db = Self {
            engine: Engine::new(),
            storage_path: path,
            encryption_key: key,
            compression: comp_algo,
            personality: Personality::new(mode_enum),
            command_history: Vec::new(),
            batch_mode: false,
            batch_ops: Vec::new(),
        };
        db.load()?;
        Ok(db)
    }

    fn create_table(&mut self, name: String, schema: Bound<'_, PyDict>) -> PyResult<PyObject> {
        validate_identifier(&name).map_err(convert_db_error)?;
        let mut native_schema = HashMap::new();
        for (field, def) in schema.iter() {
            let fname = field.extract::<String>()?;
            validate_identifier(&fname).map_err(convert_db_error)?;
            let d = def.downcast::<PyDict>()?;
            let rtype = d
                .get_item("type")?
                .ok_or_else(|| PyValueError::new_err("schema field requires type"))?
                .extract::<String>()?;
            let ftype = FieldType::from_str(&rtype).ok_or_else(|| {
                PyValueError::new_err(format!("unsupported field type {}", rtype))
            })?;
            let req = d
                .get_item("required")?
                .map(|it| it.extract::<bool>())
                .transpose()?
                .unwrap_or(false);
            let uniq = d
                .get_item("unique")?
                .map(|it| it.extract::<bool>())
                .transpose()?
                .unwrap_or(false);
            native_schema.insert(
                fname,
                FieldDef {
                    field_type: ftype,
                    required: req,
                    unique: uniq,
                },
            );
        }
        self.engine
            .create_table(&name, native_schema)
            .map_err(convert_db_error)?;
        self.persist()?;
        Python::with_gil(|py| {
            Ok(if self.personality.is_professional() {
                py.None()
            } else {
                self.personality
                    .success(&format!("Table '{}' created.", name))
                    .into_py(py)
            })
        })
    }

    fn insert(&mut self, table: String, payload: Bound<'_, PyDict>) -> PyResult<PyObject> {
        validate_identifier(&table).map_err(convert_db_error)?;
        let mut data = Map::new();
        for (k, v) in payload.iter() {
            data.insert(k.extract::<String>()?, py_to_json(v)?);
        }
        let id = self
            .engine
            .tables
            .get_mut(&table)
            .ok_or_else(|| PyKeyError::new_err(format!("table '{}' does not exist", table)))?
            .insert(data)
            .map_err(convert_db_error)?;
        self.persist()?;
        Python::with_gil(|py| {
            Ok(if self.personality.is_professional() {
                id.into_py(py)
            } else {
                self.personality
                    .success(&format!("Row inserted into '{}' (id: {}).", table, id))
                    .into_py(py)
            })
        })
    }

    fn update(&mut self, table: String, rid: u64, patch: Bound<'_, PyDict>) -> PyResult<()> {
        let mut p = Map::new();
        for (k, v) in patch.iter() {
            p.insert(k.extract::<String>()?, py_to_json(v)?);
        }
        self.engine
            .tables
            .get_mut(&table)
            .ok_or_else(|| PyKeyError::new_err(format!("table '{}' does not exist", table)))?
            .update(rid, p)
            .map_err(convert_db_error)?;
        self.persist()?;
        Ok(())
    }

    fn delete(&mut self, table: String, rid: u64) -> PyResult<()> {
        self.engine
            .tables
            .get_mut(&table)
            .ok_or_else(|| PyKeyError::new_err(format!("table '{}' does not exist", table)))?
            .delete(rid)
            .map_err(convert_db_error)?;
        self.persist()?;
        Ok(())
    }

    fn fetch_all(&self, py: Python<'_>, table: String) -> PyResult<Vec<Record>> {
        let t = self
            .engine
            .tables
            .get(&table)
            .ok_or_else(|| PyKeyError::new_err(format!("table '{}' does not exist", table)))?;
        let mut out = Vec::new();
        for (id, data) in &t.records {
            out.push(Record {
                id: *id,
                data: json_to_py(py, &Value::Object(data.clone()))?,
            });
        }
        Ok(out)
    }

    fn query(&self, py: Python<'_>, query: PyRef<'_, Query>) -> PyResult<Vec<Record>> {
        let t = self.engine.tables.get(&query.table).ok_or_else(|| {
            PyKeyError::new_err(format!("table '{}' does not exist", query.table))
        })?;
        let mut rows: Vec<(u64, Map<String, Value>)> =
            t.records.iter().map(|(id, d)| (*id, d.clone())).collect();
        for (f, e) in &query.filters {
            rows.retain(|(_, r)| r.get(f) == Some(e));
        }
        if let Some((f, d)) = &query.order_by {
            rows.sort_by(|(_, l), (_, r)| {
                let lv = l.get(f).unwrap_or(&Value::Null);
                let rv = r.get(f).unwrap_or(&Value::Null);
                let c = value_cmp(lv, rv);
                if *d {
                    c.reverse()
                } else {
                    c
                }
            });
        }
        if let Some(l) = query.limit {
            rows.truncate(l);
        }
        let mut res = Vec::new();
        for (id, r) in rows {
            res.push(Record {
                id,
                data: json_to_py(py, &Value::Object(r))?,
            });
        }
        Ok(res)
    }

    #[pyo3(signature = (text, source=None))]
    fn ingest(&mut self, text: String, source: Option<String>) -> PyResult<String> {
        let src = source.unwrap_or_else(|| "unknown".to_string());
        let word_count = text.split_whitespace().count();
        self.engine.graph_rag.ingest(&text, &src);
        self.persist()?;
        Ok(self.personality.graph_ingested(word_count))
    }

    fn graph_query(&self, query: String) -> PyResult<String> {
        let result = self.engine.graph_rag.query(&query);
        let has_results = !result.contains("No relevant information found");
        let prefix = self.personality.graph_query_result(has_results);
        Ok(format!("{}\n\n{}", prefix, result))
    }

    fn execute_sql(&mut self, py: Python<'_>, sql: String) -> PyResult<PyObject> {
        self.execute_sql_recursive(py, sql, 0)
    }

    fn execute_sql_recursive(&mut self, py: Python<'_>, sql: String, depth: usize) -> PyResult<PyObject> {
        if depth > MAX_RECURSION_DEPTH {
            return Err(PyRuntimeError::new_err("Max alias recursion depth exceeded"));
        }
        if self.batch_mode && !["COMMIT", "ROLLBACK"].contains(&sql.to_ascii_uppercase().as_str()) {
            self.batch_ops.push(sql.clone());
            return Ok("".into_py(py));
        }

        if depth == 0 { self.command_history.push(sql.clone()); }
        let toks: Vec<&str> = sql.split_whitespace().collect();
        if toks.is_empty() {
            let empty_count = self
                .command_history
                .iter()
                .filter(|s| s.trim().is_empty())
                .count() as u32;
            return Ok(self.personality.empty_input(empty_count).into_py(py));
        }

        match toks[0].to_ascii_uppercase().as_str() {
            "INGEST" => {
                if toks.len() < 2 {
                    return Err(PyValueError::new_err("INGEST requires text"));
                }
                let text = toks[1..].join(" ");
                self.ingest(text, None).map(|s| s.into_py(py))
            }
            "GRAPH_QUERY" => {
                if toks.len() < 2 {
                    return Err(PyValueError::new_err("GRAPH_QUERY requires a query"));
                }
                let q = toks[1..].join(" ");
                self.graph_query(q).map(|s| s.into_py(py))
            }
            "SHOW" | "TABLES" => Ok(self.engine.tables.keys().cloned().collect::<Vec<_>>().into_py(py)),
            "COUNT" => {
                if toks.len() < 2 {
                    return Err(PyValueError::new_err("COUNT requires a table name"));
                }
                Ok(self.engine.tables.get(toks[1]).ok_or_else(|| PyKeyError::new_err("missing table"))?.records.len().into_py(py))
            }
            "DESCRIBE" => {
                if toks.len() < 2 {
                    return Err(PyValueError::new_err("DESCRIBE requires a table name"));
                }
                let table = self.engine.tables.get(toks[1]).ok_or_else(|| PyKeyError::new_err("missing table"))?;
                let mut fields = table.schema.keys().cloned().collect::<Vec<_>>();
                fields.sort();
                Ok(fields.into_py(py))
            }
            "HISTORY" => {
                let recent = self
                    .command_history
                    .iter()
                    .rev()
                    .filter(|cmd| !cmd.trim().is_empty() && !cmd.to_uppercase().starts_with("HISTORY"))
                    .take(10)
                    .cloned()
                    .collect::<Vec<_>>();
                Ok(recent.into_py(py))
            }
            "BATCH" => {
                self.batch_mode = true;
                self.batch_ops.clear();
                Ok("Batch mode started.".into_py(py))
            }
            "COMMIT" => {
                self.batch_mode = false;
                let ops: Vec<_> = self.batch_ops.drain(..).collect();
                for operation in &ops {
                    self.execute_sql_recursive(py, operation.clone(), depth + 1)?;
                }
                Ok(self.personality.batch_committed(ops.len()).into_py(py))
            }
            "ALIAS" => {
                if toks.len() < 4 || toks[2] != "=" {
                    return Err(PyValueError::new_err("ALIAS format: ALIAS <name> = <command>"));
                }
                let alias_name = toks[1].to_ascii_lowercase();
                validate_identifier(&alias_name).map_err(convert_db_error)?;
                self.engine.aliases.insert(alias_name, toks[3..].join(" "));
                Ok("Alias created.".into_py(py))
            }
            "WHY" if toks.len() >= 5 && toks[1..4] == ["ARE", "YOU", "SO"] => Ok(self.personality.why_mean().into_py(py)),
            "ACHIEVEMENT" => Ok(self.personality.achievement_unlocked().into_py(py)),
            _ => {
                if let Some(translated) = self.engine.aliases.get(&toks[0].to_ascii_lowercase()) {
                    return self.execute_sql_recursive(py, translated.clone(), depth + 1);
                }
                if toks[0] == "DELTE" {
                    return Err(PyValueError::new_err(self.personality.typo_suggestion("DELTE", "DELETE")));
                }
                Err(PyRuntimeError::new_err(self.personality.error("unknown command")))
            }
        }
    }

    fn export_jsonl(&self, table: String, dest: String) -> PyResult<()> {
        let t = self
            .engine
            .tables
            .get(&table)
            .ok_or_else(|| PyKeyError::new_err("missing table"))?;
        let mut out = String::new();
        for (id, r) in &t.records {
            let mut m = r.clone();
            m.insert("id".into(), Value::Number((*id).into()));
            let row = serde_json::to_string(&Value::Object(m))
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
            out.push_str(&row);
            out.push('\n');
        }
        let output_path = sanitize_user_path(&dest)?;
        fs::write(output_path, out).map_err(|e| PyIOError::new_err(e.to_string()))
    }
    fn import_jsonl(&mut self, table: String, src: String) -> PyResult<usize> {
        let source_path = sanitize_user_path(&src)?;
        let d = fs::read_to_string(source_path).map_err(|e| PyIOError::new_err(e.to_string()))?;
        let t = self
            .engine
            .tables
            .get_mut(&table)
            .ok_or_else(|| PyKeyError::new_err("missing table"))?;
        let mut count = 0;
        for line in d.lines() {
            if line.trim().is_empty() {
                continue;
            }
            let mut payload: Map<String, Value> = serde_json::from_str(line).map_err(|e| PyValueError::new_err(e.to_string()))?;
            payload.remove("id");
            t.insert(payload).map_err(convert_db_error)?;
            count += 1;
        }
        self.persist()?;
        Ok(count)
    }
    fn export_sqlite(&self, table: String, dest: String) -> PyResult<()> {
        let t = self
            .engine
            .tables
            .get(&table)
            .ok_or_else(|| PyKeyError::new_err("missing table"))?;
        let output_path = sanitize_user_path(&dest)?;
        let conn = Connection::open(output_path).map_err(|e| PyIOError::new_err(e.to_string()))?;
        let mut fields: Vec<_> = t.schema.iter().collect();
        fields.sort_by_key(|f| f.0);
        let cols = fields
            .iter()
            .map(|(n, d)| format!("[{}] {}", n, d.field_type.sql_label()))
            .collect::<Vec<_>>()
            .join(", ");
        conn.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS [{}] (id INTEGER PRIMARY KEY, {})",
                table, cols
            ),
            [],
        )
        .map_err(|e| PyIOError::new_err(e.to_string()))?;
        let placeholders = (0..fields.len() + 1)
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");
        let stmt = format!(
            "INSERT INTO [{}] (id, {}) VALUES ({})",
            table,
            fields
                .iter()
                .map(|f| format!("[{}]", f.0))
                .collect::<Vec<_>>()
                .join(", "),
            placeholders
        );
        for (id, r) in &t.records {
            let mut p = vec![SqlValue::Integer(*id as i64)];
            for (fnm, _) in &fields {
                p.push(match r.get(*fnm).unwrap_or(&Value::Null) {
                    Value::Null => SqlValue::Null,
                    Value::Bool(b) => SqlValue::Integer(*b as i64),
                    Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            SqlValue::Integer(i)
                        } else if let Some(f) = n.as_f64() {
                            SqlValue::Real(f)
                        } else {
                            SqlValue::Null
                        }
                    }
                    Value::String(s) => SqlValue::Text(s.clone()),
                    _ => SqlValue::Text(r.get(*fnm).unwrap_or(&Value::Null).to_string()),
                });
            }
            conn.execute(&stmt, rusqlite::params_from_iter(p))
                .map_err(|e| PyIOError::new_err(e.to_string()))?;
        }
        Ok(())
    }

    #[pyo3(signature = (table, src, src_table=None))]
    fn import_sqlite(
        &mut self,
        table: String,
        src: String,
        src_table: Option<String>,
    ) -> PyResult<usize> {
        let sn = src_table.unwrap_or(table.clone());
        validate_identifier(&sn).map_err(convert_db_error)?;
        let source_path = sanitize_user_path(&src)?;
        let conn = Connection::open(source_path).map_err(|e| PyIOError::new_err(e.to_string()))?;
        let t = self
            .engine
            .tables
            .get_mut(&table)
            .ok_or_else(|| PyKeyError::new_err("missing table"))?;
        let mut s = conn
            .prepare(&format!("SELECT * FROM [{}]", sn))
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let cols: Vec<_> = s.column_names().into_iter().map(String::from).collect();
        let mut rows = s
            .query([])
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        let mut n = 0;
        while let Some(r) = rows
            .next()
            .map_err(|e| PyValueError::new_err(e.to_string()))?
        {
            let mut p = Map::new();
            for (i, name) in cols.iter().enumerate() {
                if name =="id" || !t.schema.contains_key(name) {
                    continue;
                }
                let value_ref = r
                    .get_ref(i)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
                p.insert(
                    name.clone(),
                    match value_ref {
                        ValueRef::Null => Value::Null,
                        ValueRef::Integer(i) => Value::Number(i.into()),
                        ValueRef::Real(f) => serde_json::Number::from_f64(f)
                            .map(Value::Number)
                            .unwrap_or(Value::Null),
                        ValueRef::Text(t) => {
                            let s = String::from_utf8_lossy(t);
                            serde_json::from_str(&s).unwrap_or(Value::String(s.to_string()))
                        }
                        _ => Value::Null,
                    },
                );
            }
            t.insert(p).map_err(convert_db_error)?;
            n += 1;
        }
        self.persist()?;
        Ok(n)
    }
}

impl Database {
    fn load(&mut self) -> PyResult<()> {
        if let Some(p) = &self.storage_path {
            if p.exists() {
                let b = fs::read(p).map_err(|e| PyIOError::new_err(e.to_string()))?;
                if b.len() < 32 {
                    return Err(PyValueError::new_err("corrupted file"));
                }
                let (c, d) = b.split_at(32);
                let mut h = Sha256::new();
                h.update(d);
                if h.finalize().as_slice() != c {
                    return Err(PyValueError::new_err("checksum mismatch"));
                }
                let mut data = d.to_vec();
                if self.encryption_key.is_some() {
                    data = self
                        .decrypt(&data)
                        .map_err(|e| PyRuntimeError::new_err(e))?;
                }
                match self.compression {
                    CompressionAlgo::Zstd => {
                        data = decode_all(&data[..]).map_err(|e| PyIOError::new_err(e.to_string()))?;
                    }
                    CompressionAlgo::Lz4 => {
                        data = decompress_size_prepended(&data[..]).map_err(|e| PyIOError::new_err(e.to_string()))?;
                    }
                    CompressionAlgo::None => {}
                }
                self.engine = bincode::deserialize(&data)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
                self.engine.rebuild_cache();
            }
        }
        Ok(())
    }
    fn persist(&self) -> PyResult<()> {
        if let Some(p) = &self.storage_path {
            let mut b = bincode::serialize(&self.engine)
                .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
            match self.compression {
                CompressionAlgo::Zstd => {
                    b = encode_all(&b[..], 3).map_err(|e| PyIOError::new_err(e.to_string()))?;
                }
                CompressionAlgo::Lz4 => {
                    b = compress_prepend_size(&b[..]);
                }
                CompressionAlgo::None => {}
            }
            if self.encryption_key.is_some() {
                b = self.encrypt(&b).map_err(|e| PyRuntimeError::new_err(e))?;
            }
            let mut h = Sha256::new();
            h.update(&b);
            let mut res = h.finalize().to_vec();
            res.extend(b);
            if let Some(prnt) = p.parent() {
                fs::create_dir_all(prnt).map_err(|e| PyIOError::new_err(e.to_string()))?;
            }
            fs::write(p, res).map_err(|e| PyIOError::new_err(e.to_string()))?;
        }
        Ok(())
    }
    fn encrypt(&self, d: &[u8]) -> Result<Vec<u8>, String> {
        let k = self.encryption_key.ok_or("no key".to_string())?;
        let c = Aes256Gcm::new_from_slice(&k).map_err(|e| e.to_string())?;
        let mut n_b = [0u8; 12];
        thread_rng().fill(&mut n_b);
        let n = Nonce::from_slice(&n_b);
        let ct = c.encrypt(n, d).map_err(|e| e.to_string())?;
        let mut out = n_b.to_vec();
        out.extend(ct);
        Ok(out)
    }
    fn decrypt(&self, d: &[u8]) -> Result<Vec<u8>, String> {
        let k = self.encryption_key.ok_or("no key".to_string())?;
        if d.len() < 12 {
            return Err("bad data".to_string());
        }
        let c = Aes256Gcm::new_from_slice(&k).map_err(|e| e.to_string())?;
        let n = Nonce::from_slice(&d[..12]);
        c.decrypt(n, &d[12..]).map_err(|e| e.to_string())
    }
}

fn sanitize_db_path(raw: &str) -> PyResult<PathBuf> {
    if raw.trim().is_empty() {
        return Err(PyValueError::new_err("path cannot be empty"));
    }
    if raw.contains('\0') {
        return Err(PyValueError::new_err("path contains invalid null byte"));
    }
    if raw.contains("..") {
        return Err(PyValueError::new_err("Potential path traversal detected."));
    }
    Ok(PathBuf::from(raw))
}

fn sanitize_user_path(raw: &str) -> PyResult<PathBuf> {
    if raw.trim().is_empty() {
        return Err(PyValueError::new_err("path cannot be empty"));
    }
    if raw.contains('\0') {
        return Err(PyValueError::new_err("path contains invalid null byte"));
    }
    if raw.contains("..") || raw.starts_with('/') {
        return Err(PyValueError::new_err("Potential path traversal detected."));
    }
    Ok(PathBuf::from(raw))
}

fn validate_identifier(i: &str) -> DbResult<()> {
    if i.is_empty() || !i.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(DbError::InvalidIdentifier(i.to_string()));
    }
    Ok(())
}
fn py_to_json(v: Bound<'_, PyAny>) -> PyResult<Value> {
    py_to_json_recursive(v, 0)
}

fn py_to_json_recursive(v: Bound<'_, PyAny>, depth: usize) -> PyResult<Value> {
    if depth > MAX_RECURSION_DEPTH {
        return Err(PyValueError::new_err("Max recursion depth exceeded in JSON conversion"));
    }
    if v.is_none() {
        return Ok(Value::Null);
    }
    if let Ok(b) = v.extract::<bool>() {
        return Ok(Value::Bool(b));
    }
    if let Ok(i) = v.extract::<i64>() {
        return Ok(Value::Number(i.into()));
    }
    if let Ok(f) = v.extract::<f64>() {
        if let Some(n) = serde_json::Number::from_f64(f) {
            return Ok(Value::Number(n));
        }
    }
    if let Ok(s) = v.extract::<String>() {
        return Ok(Value::String(s));
    }
    if let Ok(l) = v.downcast::<PyList>() {
        let mut out = Vec::new();
        for i in l {
            out.push(py_to_json_recursive(i.clone(), depth + 1)?);
        }
        return Ok(Value::Array(out));
    }
    if let Ok(d) = v.downcast::<PyDict>() {
        let mut out = Map::new();
        for (k, v) in d.iter() {
            out.insert(k.extract::<String>()?, py_to_json_recursive(v.clone(), depth + 1)?);
        }
        return Ok(Value::Object(out));
    }
    Err(PyValueError::new_err("bad type"))
}
fn json_to_py(py: Python<'_>, v: &Value) -> PyResult<PyObject> {
    Ok(match v {
        Value::Null => py.None(),
        Value::Bool(b) => b.into_py(py),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.into_py(py)
            } else if let Some(u) = n.as_u64() {
                u.into_py(py)
            } else {
                n.as_f64().unwrap_or(0.0).into_py(py)
            }
        }
        Value::String(s) => s.into_py(py),
        Value::Array(l) => {
            let mut out = Vec::new();
            for i in l {
                out.push(json_to_py(py, i)?);
            }
            out.into_py(py)
        }
        Value::Object(m) => {
            let out = PyDict::new_bound(py);
            for (k, v) in m {
                out.set_item(k, json_to_py(py, v)?)?;
            }
            out.into_py(py)
        }
    })
}
fn value_cmp(l: &Value, r: &Value) -> Ordering {
    match (l, r) {
        (Value::Number(a), Value::Number(b)) => a
            .as_f64()
            .unwrap_or(0.0)
            .partial_cmp(&b.as_f64().unwrap_or(0.0))
            .unwrap_or(Ordering::Equal),
        (Value::String(a), Value::String(b)) => a.cmp(b),
        (Value::Bool(a), Value::Bool(b)) => a.cmp(b),
        _ => Ordering::Equal,
    }
}
fn convert_db_error(e: DbError) -> PyErr {
    match e {
        DbError::MissingTable(_) | DbError::MissingField(_) | DbError::MissingRecord(_) => {
            PyKeyError::new_err(e.to_string())
        }
        _ => PyValueError::new_err(e.to_string()),
    }
}
#[pymodule]
fn _core(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Database>()?;
    m.add_class::<Query>()?;
    m.add_class::<Record>()?;
    Ok(())
}
