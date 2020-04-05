use exitfailure::ExitFailure;
use redis::Connection;
use std::collections::HashMap;

pub fn get(connection: &mut Connection, key: &str) -> Result<String, ExitFailure> {
    let value: String = redis::cmd("get").arg(key).query(connection)?;
    Ok(value)
}

pub fn get_databases(connection: &mut Connection) -> Result<i64, ExitFailure> {
    let config: HashMap<String, i64> = redis::cmd("CONFIG")
        .arg("GET")
        .arg("databases")
        .query(connection)?;
    Ok(*config.get("databases").unwrap_or(&-1))
}

pub fn select(connection: &mut Connection, db: i64) -> Result<String, ExitFailure> {
    let ok: String = redis::cmd("select").arg(db).query(connection)?;
    Ok(ok)
}
