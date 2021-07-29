use redis::{Connection, RedisError};
use std::collections::HashMap;

pub fn ttl(connection: &mut Connection, key: &str) -> Result<i64, RedisError> {
    redis::cmd("ttl").arg(key).query(connection)
}

pub fn get_type(connection: &mut Connection, key: &str) -> Result<String, RedisError> {
    redis::cmd("type").arg(key).query(connection)
}

pub fn get(connection: &mut Connection, key: &str) -> Result<String, RedisError> {
    redis::cmd("get").arg(key).query(connection)
}

pub fn hgetall(connection: &mut Connection, key: &str) -> Result<HashMap<String, i32>, RedisError> {
    redis::cmd("hgetall").arg(key).query(connection)
}

pub fn smembers(connection: &mut Connection, key: &str) -> Result<Vec<String>, RedisError> {
    redis::cmd("smembers").arg(key).query(connection)
}

pub fn lrange(
    connection: &mut Connection,
    key: &str,
    start: i64,
    stop: i64,
) -> Result<Vec<String>, RedisError> {
    redis::cmd("lrange")
        .arg(key)
        .arg(start)
        .arg(stop)
        .query(connection)
}

pub fn get_databases(connection: &mut Connection) -> Result<i64, RedisError> {
    let config: HashMap<String, i64> = redis::cmd("CONFIG")
        .arg("GET")
        .arg("databases")
        .query(connection)?;
    Ok(*config.get("databases").unwrap_or(&-1))
}

pub fn select(connection: &mut Connection, db: i64) -> Result<String, RedisError> {
    redis::cmd("select").arg(db).query(connection)
}
