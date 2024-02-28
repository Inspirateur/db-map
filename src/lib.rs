mod db_map;
pub use db_map::DBMap;

#[cfg(test)]
mod tests {
    use crate::*;

    fn is_sync<T: Sync>() { }

    fn is_send<T: Send>() { }

    #[test]
    fn test_sync() {
        is_sync::<DBMap<String, u64>>();
    }

    #[test]
    fn test_send() {
        is_send::<DBMap<String, u64>>();
    }

    #[test]
    fn test_methods() {
        let db_map: DBMap<String, u64> = DBMap::new("db_map.db").unwrap();
        db_map.insert("Test".to_string(), 42).unwrap();
        db_map.insert("Hello".to_string(), 1).unwrap();
        db_map.insert("World".to_string(), 1).unwrap();
        assert_eq!(db_map.get("Test".to_string()).unwrap(), 42);
        assert_eq!(db_map.get_keys(1).unwrap(), ["Hello".to_string(), "World".to_string()]);
    }
}