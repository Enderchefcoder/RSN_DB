#[cfg(test)]
mod tests {
    use crate::alive::AliveState;
    use crate::graph_rag::GraphRagEngine;
    use crate::personality::{Mode, Personality};
    use crate::{sanitize_relative_path, validate_identifier, DbError, Engine, FieldDef, FieldType, Table};
    use serde_json::{json, Map};
    use std::collections::HashMap;

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
    fn engine_has_alive_state() {
        let engine = Engine::new();
        assert_eq!(engine.alive.commands_total, 0);
    }

    #[test]
    fn graph_rag_ingest_and_query() {
        let mut g = GraphRagEngine::new();
        g.ingest("Alice works at RSN DB.", "src");
        assert!(!g.query("Alice").is_empty());
    }

    #[test]
    fn personality_snark_uses_extra_pool() {
        let p = Personality::new(Mode::Snarky);
        let msg = p.success("done");
        assert!(msg.contains('✓'));
    }

    #[test]
    fn alive_mood_and_pulse() {
        let mut a = AliveState::default();
        a.on_success();
        assert!(!a.pulse(Mode::Professional).is_empty());
    }
}
