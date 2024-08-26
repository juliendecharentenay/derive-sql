# Introduction

Based on the Postgres - Rust Cookbook Aggregate data, this story uses the Museum of Modern Art (MOMA) Collection database available on github to demonstrate how to use `derive_sql` to collect the records in an SQL database and subsequently query these records.

# Instructions:

Compilation:
```
cargo build
```

Run and show help:
```
cargo run -- help 
```

Run using SQLite:
```
# Collect data in SQLite database file test.db3
cargo run -- -d sqlite -f test.db3 collect

# Query the top country by number of artists (from the collected data)
cargo run -- -d sqlite -f test.db3 query_nationalities

# Query the top country by number of artworks (from the collected data)
cargo run -- -d sqlite -f test.db3 query_artwork_nationalities
```

Run using MySQL database - you need to have setup MySQL with a user and a database:
```
# Collect data in SQLite database file test.db3
cargo run -- -d mysql -f localhost -u username -p password -n database_name collect

# Query the top country by number of artists (from the collected data)
cargo run -- -d mysql -f localhost -u username -p password -n database_name query_nationalities

# Query the top country by number of artworks (from the collected data)
cargo run -- -d mysql -m localhost -u username -p password -n database_name query_artwork_nationalities
```


