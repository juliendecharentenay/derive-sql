# derive-sql

The project contains the following:

- `derive-sql`: the root of the `derive-sql` library as published on crates.io. Defines the trait `Sqlable`.
- `extras/derive-sql-sqlite`: the package with the implementation of the `DeriveSqlite` macro that implement the `Sqlable` trait
based on a struct with named fields.
- `extras/derive-sql-mysql`: the package with the implementation of the `DeriveMysql` macro that implement the `Sqlable` trait for MySQL based on a struct with named fields.

## Publishing

Use script `publish.sh` to publish all crates - remember to update `OLD_VERSION` and `NEW_VERSION` in the file before running!

