use redis::Connection;
use std::collections::HashMap;

pub fn get(connection: &mut Connection, key: &str) -> String {
    let value: String = redis::cmd("get").arg(key).query(connection).unwrap();
    value
}

pub fn get_databases(connection: &mut Connection) -> i64 {
    let config: HashMap<String, i64> = redis::cmd("CONFIG")
        .arg("GET")
        .arg("databases")
        .query(connection)
        .unwrap();
    *config.get("databases").unwrap_or(&-1)
}

pub fn select(connection: &mut Connection, db: i64) -> String {
    let ok: String = redis::cmd("select").arg(db).query(connection).unwrap();
    ok
}
