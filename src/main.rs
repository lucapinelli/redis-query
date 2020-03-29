use quicli::prelude::*;
use redis::Commands;
use redis::Connection;
use std::collections::HashMap;
use structopt::StructOpt;

/// Query multiples Redis databases
#[derive(Debug, StructOpt)]
struct Cli {
    /*
     * Flags
     */
    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,

    // /// Shows the values associated with the keys
    // #[structopt(long = "show-value", short = "l")]
    // show_value: bool,

    /*
     * Options
     */
    // /// Server hostname
    // #[structopt(long = "hostname", short = "h", default_value = "127.0.0.1")]
    // hostname: String,
    //
    // /// Server port
    // #[structopt(long = "port", short = "p", default_value = "6379")]
    // port: u32,

    // /// Pattern mode
    // #[structopt(long = "pattern-mode", short = "m", default_value = "standard")]
    // pattern_mode: String,
    //
    // /// Select the databese to query
    // #[structopt(long = "db", short = "d", default_value = "all")]
    // db: String,

    /*
     * Required Parameters
     */
    /// The keys' query
    query: String,
}

fn get_databases(connection: &mut Connection) -> i64 {
    let config: HashMap<String, i64> = redis::cmd("CONFIG")
        .arg("GET")
        .arg("databases")
        .query(connection)
        .unwrap();
    *config.get("databases").unwrap_or(&-1)
}

fn select(connection: &mut Connection, db: i64) -> String {
    let ok: String = redis::cmd("select").arg(db).query(connection).unwrap();
    ok
}

fn main() -> CliResult {
    let args = Cli::from_args();

    println!("args = {:?}", args);

    let client = redis::Client::open("redis://localhost:6379").unwrap();
    let mut connection = client.get_connection().unwrap();
    let databases = get_databases(&mut connection);
    println!("databases = {}", databases);
    (0..databases).for_each(|db| {
        select(&mut connection, db);
        let keys: Vec<String> = connection.keys(&args.query).unwrap();
        if !keys.is_empty() {
            println!("DB({}) {}", db, keys.join(", "));
        }
    });

    Ok(())
}
