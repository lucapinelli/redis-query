use clap::Parser;

/// Dev tool to search keys in a Redis cache.
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    /*
     * Flags
     */
    /// Shows the values associated with the keys
    #[clap(long = "show-value", short = 'v')]
    pub show_value: bool,

    /// Shows the values associated with the keys
    #[clap(long = "show-ttl", short = 't')]
    pub show_ttl: bool,

    /// When enabled returns all the keys that contains the query (case-insensitive)
    #[clap(long = "easy-search", short = 'e')]
    pub easy_search: bool,

    /*
     * Options
     */
    /// Server hostname
    #[clap(long = "hostname", short = 'h', default_value = "127.0.0.1")]
    pub hostname: String,

    /// Server port
    #[clap(long = "port", short = 'p', default_value = "6379")]
    pub port: u32,

    /// Select the database to query.
    /// If no database is specified the tool will search in all the available databases.
    #[clap(long = "db", short = 'd', default_value = "-1")]
    pub db: i64,

    /*
     * Required Parameters
     */
    /// The pattern to use to search the keys
    pub query: String,
}
