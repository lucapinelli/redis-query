use quicli::prelude::*;
use redis::Commands;
use structopt::StructOpt;

mod util;
use crate::util::cli::Cli;
use crate::util::redis::*;

fn main() -> CliResult {
    let args = Cli::from_args();

    let connection_string = format!("redis://{}:{}", args.hostname, args.port);

    let client = redis::Client::open(connection_string).unwrap();
    let mut connection = client.get_connection().unwrap();
    let databases = get_databases(&mut connection);

    let query = if args.easy_search {
        format!(
            "*{}*",
            args.query
                .chars()
                .map(|c| format!("[{}{}]", c, c.to_uppercase()))
                .collect::<String>()
        )
    } else {
        args.query.clone()
    };

    let mut search = |db| {
        select(&mut connection, db);

        let keys: Vec<String> = connection.keys(&query).unwrap();
        if !keys.is_empty() {
            if args.show_value {
                keys.iter()
                    .for_each(|key| println!("DB({}) {} = {}", db, key, get(&mut connection, key)));
            } else {
                println!("DB({}) {}", db, keys.join(", "));
            }
        }
    };

    if args.db >= 0 {
        search(args.db);
    } else {
        (0..databases).for_each(search);
    }

    Ok(())
}
