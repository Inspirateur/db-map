# db-map
A Send&Sync typed key-value store - persisted to the disk with SQLite - with the following methods:
- `insert(key, value)`
- `get(key) -> value`
- `get_keys(value) -> [keys]`

```rust
use db_map::DBMap;
use anyhow::Result;

fn db_map_demo() -> Result<()> {
    let db_map: DBMap<String, u64> = DBMap::new("db_map.db")?;
    db_map.insert("Test".to_string(), 42)?;
    db_map.insert("Hello".to_string(), 1)?;
    db_map.insert("World".to_string(), 1)?;
    assert_eq!(db_map.get("Test".to_string())?, 42);
    assert_eq!(db_map.get_keys(1)?, ["Hello".to_string(), "World".to_string()]);
    Ok(())
}
```