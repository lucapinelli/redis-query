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
    let databases = get_databases(&mut connection);

    let query = if easy_search {
        format!(
            "*{}*",
            query
                .chars()
                .map(|c| format!("[{}{}]", c, c.to_uppercase()))
                .collect::<String>()
        )
    } else {
        query
    };

    let mut search = |db| {
        select(&mut connection, db);

        let keys: Vec<String> = connection.keys(&query).unwrap();
        if !keys.is_empty() {
            if show_value {
                keys.iter()
                    .for_each(|key| println!("DB({}) {} = {}", db, key, get(&mut connection, key)));
            } else {
                println!("DB({}) {}", db, keys.join(", "));
            }
        }
    };

    if db >= 0 {
        search(db);
    } else {
        (0..databases).for_each(search);
    }

    Ok(())
}
