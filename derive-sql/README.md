# derive-sql

This project define an approach to interact with SQL database in Rust [currently only SQLite supported]. 

A trait `Sqlable` is defined in the crate with the following functions to interact with the database:
- `count` to provide a count of the number of items in the table.
- `select` to return an array of the items in the table.
- `insert` to insert a new item in the table.
- `update` to update an existing item(s) with the values of the provided item.
- `delete` to delete items in the table.
- `delete_table` to drop the table.

A procedural macro `DeriveSqlite` is available under the optional feature `sqlite`. The procedural macro can 
be applied to a struct with named fields to implement the `Sqlable` trait.

# How to Use
The procedural macro is currently tied to `rusqlite` that needs to be added to your `Cargo.toml`. If not added, your
project will not compile.

To use this project procedural macro, add the following in your `Cargo.toml`:

```
[dependencies]
derive-sql = { version = "0.5", features = [ 'sqlite' ] }
```

And annotate your struct as follows:
```
use derive_sql::{Sqlable, DeriveSqlite};

#[derive(DeriveSqlite)]
struct Person {
  id: 32,
  name: String,
}
```

And use the generated functions:

* Review the documentation pages;
* Generate documentation for the generated functions using `cargo doc --open`

Checkout the example tests using in-memory SQLite database in the `extras/derive-sql-sqlite/examples` folder:
```
cargo run --example simple --features sqlite
cargo run --example attributes --features sqlite
```


# License
This project is licensed under [MIT license](LICENSE).


