use clap::Parser;

/// Dev tool to search keys in a Redis cache.
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Cli {
    /*
     * Flags
     */
    /// Shows the values associated with the keys
    #[clap(short = 'v', long)]
    pub show_value: bool,

    /// Shows the TTL associated with the keys
    #[clap(short = 't', long)]
    pub show_ttl: bool,

    /// Match the keys using contains (case-insensitive)
    #[clap(short = 'c', long = "contains")]
    pub use_contains: bool,

    /// Colorizes the output
    #[clap(long = "color")]
    pub use_color: bool,

    /*
     * Options
     */
    /// Server hostname
    #[clap(short, long, default_value = "127.0.0.1")]
    pub hostname: String,

    /// Server port
    #[clap(short, long, default_value = "6379")]
    pub port: u32,

    /// Select the database to query.
    /// If no database is specified the tool will search in all the available databases.
    #[clap(short, long, default_value = "-1")]
    pub db: i64,

    /*
     * Required Parameters
     */
    /// The pattern used to filter the keys as defined in the Redis doc (https://redis.io/commands/keys)
    pub query: String,
}
