#[cfg(test)]
mod tests {
    use crate::graph_rag::GraphRagEngine;
    use crate::personality::{Mode, Personality};
    use crate::{sanitize_relative_path, validate_identifier, DbError, Engine, FieldDef, FieldType, Table};
    use serde_json::{json, Map};

    #[test]
    fn field_type_from_str_and_coerce() {
        assert_eq!(FieldType::from_str("STRING"), Some(FieldType::String));
        assert!(FieldType::Integer.coerce(json!("42")).is_some());
    }

    #[test]
    fn validate_identifier_rejects_bad() {
        assert!(validate_identifier("ok_table").is_ok());
        assert!(validate_identifier("bad-name").is_err());
    }

    #[test]
    fn sanitize_relative_path_blocks_traversal() {
        assert!(sanitize_relative_path("../etc/passwd", true, false).is_err());
        assert!(sanitize_relative_path("safe/file.jsonl", true, false).is_ok());
    }

    #[test]
    fn table_unique_violation() {
        use std::collections::HashMap;
        let mut schema = HashMap::new();
        schema.insert(
            "email".to_string(),
            FieldDef {
                field_type: FieldType::String,
                required: true,
                unique: true,
            },
        );
        let mut table = Table::new(schema);
        let mut row = Map::new();
        row.insert("email".to_string(), json!("a@x.com"));
        assert!(table.insert(row.clone()).is_ok());
        assert!(matches!(table.insert(row), Err(DbError::UniqueViolation(_))));
    }

    #[test]
    fn graph_rag_ingest_and_query() {
        let mut g = GraphRagEngine::new();
        g.ingest("Alice works at RSN DB.", "src");
        let out = g.query("Alice");
        assert!(!out.is_empty());
    }

    #[test]
    fn personality_snark_pick_non_empty() {
        let p = Personality::new(Mode::Snarky);
        let msg = p.success("done");
        assert!(msg.contains('✓'));
    }
}
