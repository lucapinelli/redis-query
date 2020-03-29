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

    let mut search = |db| {
        select(&mut connection, db);
        let keys: Vec<String> = connection.keys(&args.query).unwrap();
        if !keys.is_empty() {
            if args.show_value {
                println!("# DB {}", db);
                keys.iter()
                    .for_each(|key| println!("{} = {}", key, get(&mut connection, key)));
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
