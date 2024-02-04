# derive-sql

The project contains the following:

- `derive-sql`: the root of the `derive-sql` library as published on crates.io. Defines the trait `Sqlable`.
- `extras/derive-sql-sqlite`: the package with the implementation of the `DeriveSqlite` macro that implement the `Sqlable` trait
based on a struct with named fields.
- `extras/derive-sql-mysql`: the package with the implementation of the `DeriveMysql` macro that implement the `Sqlable` trait for MySQL based on a struct with named fields.

## Publishing

Process to work through publishing all crates:

```
# Modify Cargo.toml to use version in place of path
vi extras/derive-sql-mysql/Cargo.toml extras/derive-sql-sqlite/Cargo.toml derive-sql/Cargo.toml
git add extras/derive-sql-mysql/Cargo.toml extras/derive-sql-sqlite/Cargo.toml derive-sql/Cargo.toml
git commit -m "Pre-release change from path to version"
git push

git tag -a v0.7.0 -m "Version 0.7.0"
git push origin v0.7.0

(
  cd extras/derive-sql-mysql
  cargo publish
)

(
  cd extras/derive-sql-sqlite
  cargo publish
)

(
  cd derive-sql
  cargo publish
)
```

