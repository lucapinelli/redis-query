use quicli::prelude::*;
use structopt::StructOpt;

///
/// Searches keys in multiple Redis' databases
#[derive(Debug, StructOpt)]
pub struct Cli {
    /*
     * Flags
     */
    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    pub verbosity: Verbosity,

    /// Shows the values associated with the keys
    #[structopt(long = "show-value", short = "l")]
    pub show_value: bool,

    /// When the "easy search" is enabled the query will match all the keys that includes
    /// the give string (case-insensitive).
    #[structopt(long = "easy-search", short = "e")]
    pub easy_search: bool,

    /*
     * Options
     */
    /// Server hostname
    #[structopt(long = "hostname", short = "h", default_value = "127.0.0.1")]
    pub hostname: String,

    /// Server port
    #[structopt(long = "port", short = "p", default_value = "6379")]
    pub port: u32,

    /// Select the database to query.
    /// If no database is specified the tool will search in all the available databases.
    #[structopt(long = "db", short = "d", default_value = "-1")]
    pub db: i64,

    /*
     * Required Parameters
     */
    /// The pattern to use to search the keys
    pub query: String,
}
