

```
A tool to search keys in multiple Redis' databases

USAGE:
    redis-query [OPTIONS] <QUERY>

ARGS:
    <QUERY>    The pattern used to filter the keys as defined in the Redis doc
               (https://redis.io/commands/keys)

OPTIONS:
    -c, --contains               Match the keys using contains (case-insensitive)
        --color                  Colorizes the output
    -d, --db <DB>                Select the database to query. If no database is specified the tool
                                 will search in all the available databases [default: -1]
    -h, --hostname <HOSTNAME>    Server hostname [default: 127.0.0.1]
        --help                   Print help information
    -p, --port <PORT>            Server port [default: 6379]
    -t, --show-ttl               Shows the TTL associated with the keys
    -v, --show-value             Shows the values associated with the keys
    -V, --version                Print version information
```

# Examples

Assuming that Redis contains 2 databases with the following content:

**Database 0**

Key | Value
--- | -----
aThing | Derby Rambla
person_12 | luca
person_23 | anna
something | a value

**Database 1**

Key | Value
--- | -----
something_else | another value
Bookmarks_97 | https://github.com/lucapinelli/redis-query
Bookmarks_163 | https://redis.io/commands#generic
bookmarks_count | 2

To get the keys in all the databases:

```bash
$ redis-query "*"
DB(0) aThing, person_12, person_23, something
DB(1) Bookmarks_97, Bookmarks_163, something_else, bookmarks_count
```

To search, in all the databases, the keys that matches the pattern 'something*':

```bash
$ redis-query "something*"
DB(0) something
DB(1) something_else
```

To show the values associated with the keys:

```bash
$ redis-query --show-value "something*"
DB(0) something = a value
DB(1) something_else = another value
```

To search the keys only in the database 1:

```bash
$ redis-query --db 1 "something*"
DB(1) something_else
```

To search the keys that contains "bookmark" (case insensitive):

```bash
$ redis-query --contains "bookmark"
DB(1) Bookmarks_97, Bookmarks_163, bookmarks_count
# this is the same of using:
$ redis-query "*[Bb][Oo][Oo][Kk][Mm][Aa][Rr][Kk]*"
DB(1) Bookmarks_97, Bookmarks_163, bookmarks_count
```

# Install

## Using Cargo

```bash
cargo install redis-query
```
