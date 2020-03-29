use quicli::prelude::*;
use redis::Commands;
use structopt::StructOpt;

mod util;
use crate::util::cli::Cli;
use crate::util::redis::*;

fn main() -> CliResult {
    let args = Cli::from_args();

    let connection_string = format!("redis://{}:{}", &args.hostname, &args.port);

    let client = redis::Client::open(connection_string).unwrap();
    let mut connection = client.get_connection().unwrap();
    let databases = get_databases(&mut connection);
    (0..databases).for_each(|db| {
        select(&mut connection, db);
        let keys: Vec<String> = connection.keys(&args.query).unwrap();
        if !keys.is_empty() {
            println!("DB({}) {}", db, keys.join(", "));
        }
    });

    Ok(())
}
