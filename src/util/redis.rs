use failure::{Error, ResultExt};
use redis::{Connection, RedisError};
use std::collections::HashMap;

pub fn ttl(connection: &mut Connection, key: &str) -> Result<i64, RedisError> {
    redis::cmd("ttl").arg(key).query(connection)
}

pub fn get_type(connection: &mut Connection, key: &str) -> Result<String, RedisError> {
    redis::cmd("type").arg(key).query(connection)
}

// string
pub fn get(connection: &mut Connection, key: &str) -> Result<String, RedisError> {
    redis::cmd("get").arg(key).query(connection)
}

// list
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

// set
pub fn smembers(connection: &mut Connection, key: &str) -> Result<Vec<String>, RedisError> {
    redis::cmd("smembers").arg(key).query(connection)
}

// zset (sorted set)
pub fn zrange(
    connection: &mut Connection,
    key: &str,
    start: i64,
    stop: i64,
) -> Result<Vec<String>, RedisError> {
    redis::cmd("zrange")
        .arg(key)
        .arg(start)
        .arg(stop)
        .query(connection)
}

// hash
pub fn hgetall(connection: &mut Connection, key: &str) -> Result<HashMap<String, i32>, RedisError> {
    redis::cmd("hgetall").arg(key).query(connection)
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

pub fn format_value(connection: &mut Connection, key: &str) -> Result<String, Error> {
    let value_type = get_type(connection, key)?;
    let value = match value_type.as_str() {
        "string" => Some(
            get(connection, key)
                .with_context(|_| format!("using get on the key `{}` ({})", key, value_type))?,
        ),
        "list" => Some(format!(
            "{:?}",
            lrange(connection, key, 0, -1)
                .with_context(|_| format!("using lrange on the key `{}` ({})", key, value_type))?
        )),
        "set" => Some(format!(
            "{:?}",
            smembers(connection, key).with_context(|_| format!(
                "using smembers on the key `{}` ({})",
                key, value_type
            ))?
        )),
        "zset" => Some(format!(
            "{:?}",
            zrange(connection, key, 0, -1)
                .with_context(|_| format!("using zrange on the key `{}` ({})", key, value_type))?
        )),
        "hash" => Some(format!(
            "{:?}",
            hgetall(connection, key)
                .with_context(|_| format!("using hgetall on the key `{}` ({})", key, value_type))?
        )),
        _ => None,
    };

    match value {
        Some(text) => Ok(text),
        None => Err(failure::err_msg(format!(
            "type `{}` not supported by the redis-query tool.",
            value_type
        ))),
    }
}
