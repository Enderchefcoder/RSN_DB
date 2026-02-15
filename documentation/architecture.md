# RSN_DB Architecture

RSN_DB uses a **single in-memory engine** with snapshots for history control.

## Layers

1. **Rust engine (`Engine`)**
   - Owns tables, edges, key-value store, and checkpoints.
   - Enforces schema validation and relation integrity.

2. **Python binding (`RSNDatabase`)**
   - Converts Python values into strongly typed Rust `Value` variants.
   - Exposes an API shaped for direct scripting, tests, and notebooks.

3. **Persistence boundary**
   - `save(path)` serializes `core` state and named checkpoints.
   - `load(path)` restores state and resets undo/redo stacks.

## Data model

- `Table`
  - Name, schema, flexibility mode
  - Auto-generated IDs (for example `use_00001`)
- `Row`
  - `id` plus `fields` map
- `Edge`
  - `from_table`, `from_id`, `label`, `to_table`, `to_id`

## Query model

Conditions are represented as a tuple:

```python
(field, operator, value)
```

Supported operators:

- Equality: `=`, `==`, `!=`
- Numeric: `>`, `<`, `>=`, `<=`
- Composite: `contains` (strings and arrays)

## Transaction-ish controls

Each mutating operation creates an internal snapshot, enabling:

- `undo()` / `redo()` for stepwise history
- Named checkpoints with `checkpoint(name)` and `rollback_to(name)`

This is snapshot-based, so memory usage grows with mutation volume.
