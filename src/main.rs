use exitfailure::ExitFailure;
use failure::ResultExt;
use redis::Commands;
use structopt::StructOpt;

mod util;
use crate::util::cli::Cli;
use crate::util::redis::*;

fn main() -> Result<(), ExitFailure> {
    let Cli {
        hostname,
        port,
        easy_search,
        query,
        show_ttl,
        show_value,
        db,
    } = Cli::from_args();

    let connection_string = format!("redis://{}:{}", hostname, port);

    let client = redis::Client::open(connection_string.clone())
        .with_context(|_| format!("opening the Redis client using `{}`", connection_string))?;

    let mut connection = client
        .get_connection()
        .with_context(|_| "connecting to the Redis server")?;
    let databases =
        get_databases(&mut connection).with_context(|_| "getting the number of databases")?;

    let query = if easy_search {
        let insensitive_query = query
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    format!("[{}{}]", c.to_lowercase(), c.to_uppercase())
                } else {
                    c.to_string()
                }
            })
            .collect::<String>();
        format!("*{}*", insensitive_query)
    } else {
        query
    };

    let mut search = |db| -> Result<(), ExitFailure> {
        select(&mut connection, db).with_context(|_| format!("selecting the database {}", db))?;

        let keys: Vec<String> = connection.keys(&query)?;
        if !keys.is_empty() {
            if show_value || show_ttl {
                keys.iter().try_for_each(|key| -> Result<(), ExitFailure> {
                    let value = if show_value {
                        let value_type = get_type(&mut connection, key)?;
                        match value_type.as_str() {
                            "string" => format!(
                                "= {}",
                                get(&mut connection, key).with_context(|_| format!(
                                    "using get on the key `{}` ({})",
                                    key, value_type
                                ))?
                            ),
                            "list" => format!(
                                "= {:?}",
                                lrange(&mut connection, key, 0, -1).with_context(|_| format!(
                                    "using lrange on the key `{}` ({})",
                                    key, value_type
                                ))?
                            ),
                            "set" => format!(
                                "= {:?}",
                                smembers(&mut connection, key).with_context(|_| format!(
                                    "using smembers on the key `{}` ({})",
                                    key, value_type
                                ))?
                            ),
                            "zset" => format!(
                                "= {:?}",
                                zrange(&mut connection, key, 0, -1).with_context(|_| format!(
                                    "using zrange on the key `{}` ({})",
                                    key, value_type
                                ))?
                            ),
                            "hash" => format!(
                                "= {:?}",
                                hgetall(&mut connection, key).with_context(|_| format!(
                                    "using hgetall on the key `{}` ({})",
                                    key, value_type
                                ))?
                            ),
                            _ => format!(":: type `{}` not supported.", value_type),
                        }
                    } else {
                        String::new()
                    };
                    let ttl = if show_ttl {
                        format!(
                            "[ttl {}]",
                            ttl(&mut connection, key)
                                .with_context(|_| format!("accessing the key `{}`", key))?
                        )
                    } else {
                        String::new()
                    };
                    println!("DB({}) {} {} {}", db, key, value, ttl);
                    Ok(())
                })?;
            } else {
                println!("DB({}) {}", db, keys.join(", "));
            }
        }
        Ok(())
    };

    if db >= 0 {
        search(db)?;
    } else {
        (0..databases).try_for_each(search)?;
    }

    Ok(())
}
