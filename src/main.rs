use exitfailure::ExitFailure;
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
        show_value,
        db,
    } = Cli::from_args();

    let connection_string = format!("redis://{}:{}", hostname, port);

    let client = redis::Client::open(connection_string)?;
    let mut connection = client.get_connection()?;
    let databases = get_databases(&mut connection)?;

    let query = if easy_search {
        let insensitive_query = query
            .chars()
            .map(|c| format!("[{}{}]", c, c.to_uppercase()))
            .collect::<String>();
        format!("*{}*", insensitive_query)
    } else {
        query
    };

    let mut search = |db| -> Result<(), ExitFailure> {
        select(&mut connection, db)?;

        let keys: Vec<String> = connection.keys(&query)?;
        if !keys.is_empty() {
            if show_value {
                keys.iter().try_for_each(|key| -> Result<(), ExitFailure> {
                    println!("DB({}) {} = {}", db, key, get(&mut connection, key)?);
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
