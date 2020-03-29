use quicli::prelude::*;
use structopt::StructOpt;

/// Searches keys in multiple Redis databases
#[derive(Debug, StructOpt)]
pub struct Cli {
    /*
     * Flags
     */
    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    pub verbosity: Verbosity,

    // /// Shows the values associated with the keys
    // #[structopt(long = "show-value", short = "l")]
    // show_value: bool,

    /*
     * Options
     */
    /// Server hostname
    #[structopt(long = "hostname", short = "h", default_value = "127.0.0.1")]
    pub hostname: String,

    /// Server port
    #[structopt(long = "port", short = "p", default_value = "6379")]
    pub port: u32,

    // /// Pattern mode
    // #[structopt(long = "pattern-mode", short = "m", default_value = "standard")]
    // pattern_mode: String,
    //
    /// Select the databese to query
    #[structopt(long = "db", short = "d", default_value = "-1")]
    pub db: i64,

    /*
     * Required Parameters
     */
    /// The keys' query
    pub query: String,
}
