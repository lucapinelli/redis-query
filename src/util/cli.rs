use structopt::StructOpt;

/// Dev tool to search keys in a Redis cache.
#[derive(StructOpt)]
pub struct Cli {
    /*
     * Flags
     */
    /// Shows the values associated with the keys
    #[structopt(long = "show-value", short = "v")]
    pub show_value: bool,

    /// Shows the values associated with the keys
    #[structopt(long = "show-ttl", short = "t")]
    pub show_ttl: bool,

    /// When enabled returns all the keys that contains the query (case-insensitive)
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
