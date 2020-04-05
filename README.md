

```
Searches keys in multiple Redis' databases

USAGE:
    redis-query [FLAGS] [OPTIONS] <query>

FLAGS:
    -e, --easy-search    
            When the "easy search" is enabled the query will match all the keys that includes the give string (case-
            insensitive).

        --help           
            Prints help information

    -l, --show-value     
            Shows the values associated with the keys

    -V, --version        
            Prints version information

    -v, --verbosity      
            Pass many times for more log output

            By default, it'll only report errors. Passing `-v` one time also prints warnings, `-vv` enables info
            logging, `-vvv` debug, and `-vvvv` trace.

OPTIONS:
    -d, --db <db>                
            Select the database to query. If no database is specified the tool will search in all the available
            databases. [default: -1]

    -h, --hostname <hostname>    
            Server hostname [default: 127.0.0.1]

    -p, --port <port>            
            Server port [default: 6379]

ARGS:
    <query>    
            The pattern to use to search the keys
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

```
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
$ redis-query -l "something*"
DB(0) something = a value
DB(1) something_else = another value
```

To search the keys only in the database 1:

```bash
$ redis-query -d 1 "something*"
DB(1) something_else
```

To search the keys that contains "bookmark" (case insensitive):

```bash
$ redis-query -e "bookmark"
DB(1) Bookmarks_97, Bookmarks_163, bookmarks_count
```

# Install

## Using Cargo

```bash
cargo install redis-query
```
