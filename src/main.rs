use clap::Parser;
use colored::Colorize;
use exitfailure::ExitFailure;
use failure::ResultExt;
use redis::Commands;

mod util;
use crate::util::cli::Cli;
use crate::util::redis::*;

fn main() -> Result<(), ExitFailure> {
    let Cli {
        hostname,
        port,
        use_contains,
        use_color,
        query,
        show_ttl,
        show_value,
        db,
    } = Cli::parse();

    let connection_string = format!("redis://{}:{}", hostname, port);

    let client = redis::Client::open(connection_string.clone())
        .with_context(|_| format!("opening the Redis client using `{}`", connection_string))?;

    let mut connection = client
        .get_connection()
        .with_context(|_| "connecting to the Redis server")?;
    let databases =
        get_databases(&mut connection).with_context(|_| "getting the number of databases")?;

    let query = if use_contains {
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
                    let value_prefix = if show_value { "=" } else { "" };

                    let value = if show_value {
                        format_value(&mut connection, key).with_context(|_| {
                            format!("formatting the value of the key `{}`", key)
                        })?
                    } else {
                        String::new()
                    };

                    let ttl = if show_ttl {
                        format!(
                            "[ttl {}]",
                            ttl(&mut connection, key).with_context(|_| format!(
                                "accessing the TTL of the key `{}`",
                                key
                            ))?
                        )
                    } else {
                        String::new()
                    };

                    if use_color {
                        println!(
                            "{} {} {} {} {}",
                            format!("DB({})", db).blue(),
                            key.green(),
                            value_prefix,
                            value.cyan(),
                            ttl.yellow()
                        );
                    } else {
                        println!("DB({}) {} {} {} {}", db, key, value_prefix, value, ttl);
                    }
                    Ok(())
                })?;
            } else if use_color {
                println!(
                    "{} {}",
                    format!("DB({})", db).blue(),
                    keys.join(", ").green()
                );
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
