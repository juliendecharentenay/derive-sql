# derive-sql

This project provides a Rust procedural macro to help with using SQL database in Rust. The procedural
macro build methods on a given struct to allow an SQL table to be created, populated and items queried,
added, modified and deleted.

The following Rust SQL wrapper are supported:
* [`rusqlite`](https://crates.io/crates/rusqlite)

# How to Use
The procedural macro is currently tied to `rusqlite` that needs to be added to your `Cargo.toml`. If not added, your
project will not compile.

To use this project procedural macro, add the following in your `Cargo.toml`:

```
[dependencies]
derive-sql = { version = "0.2" }
```

And annotate your struct as follows:
```
use derive_sql::DeriveSql;

#[derive(DeriveSql)]
struct Person {
  id: 32,
  name: String,
}
```

And use the generated functions:

* Review the documentation pages;
* Generate documentation for the generated functions using `cargo doc --open`

Checkout the example tests using in-memory SQLite database in the `tests` folder.

# License
This project is licensed under [MIT license](LICENSE).


