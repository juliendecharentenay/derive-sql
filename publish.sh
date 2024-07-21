#!/bin/bash

export OLD_VERSION=0.11.1
export NEW_VERSION=0.11.2
export VERSION=v$NEW_VERSION

# Modify Cargo.toml to use version in place of path
sed -s -i -e 's/^derive-sql/## Dev derive-sql/' extras/derive-sql-common/Cargo.toml extras/derive-sql-mysql/Cargo.toml extras/derive-sql-sqlite/Cargo.toml extras/derive-sql-statement/Cargo.toml derive-sql/Cargo.toml
sed -s -i -e 's/^## Pub derive-sql/derive-sql/' extras/derive-sql-common/Cargo.toml extras/derive-sql-mysql/Cargo.toml extras/derive-sql-sqlite/Cargo.toml extras/derive-sql-statement/Cargo.toml derive-sql/Cargo.toml

## Modify vesion number
sed -s -i -e "s/^version = \"$OLD_VERSION\"/version = \"$NEW_VERSION\"/" extras/derive-sql-common/Cargo.toml extras/derive-sql-mysql/Cargo.toml extras/derive-sql-sqlite/Cargo.toml extras/derive-sql-statement/Cargo.toml derive-sql/Cargo.toml

git add extras/derive-sql-common/Cargo.toml extras/derive-sql-mysql/Cargo.toml extras/derive-sql-sqlite/Cargo.toml extras/derive-sql-statement/Cargo.toml derive-sql/Cargo.toml
git commit -m "Pre-release change from path to version"
git push

git tag -a $VERSION -m "Version $NEW_VERSION"
git push origin $VERSION

(
  cd extras/derive-sql-common
  cargo publish
)

(
  cd extras/derive-sql-mysql
  cargo publish
)

(
  cd extras/derive-sql-sqlite
  cargo publish
)

(
  cd extras/derive-sql-statement
  cargo publish
)

(
  cd derive-sql
  cargo publish
)

# Modify Cargo.toml to use path in place of version
sed -s -i -e 's/^derive-sql/## Pub derive-sql/' extras/derive-sql-common/Cargo.toml extras/derive-sql-mysql/Cargo.toml extras/derive-sql-sqlite/Cargo.toml extras/derive-sql-statement/Cargo.toml derive-sql/Cargo.toml
sed -s -i -e 's/^## Dev derive-sql/derive-sql/' extras/derive-sql-common/Cargo.toml extras/derive-sql-mysql/Cargo.toml extras/derive-sql-sqlite/Cargo.toml extras/derive-sql-statement/Cargo.toml derive-sql/Cargo.toml

git add extras/derive-sql-common/Cargo.toml extras/derive-sql-mysql/Cargo.toml extras/derive-sql-sqlite/Cargo.toml extras/derive-sql-statement/Cargo.toml derive-sql/Cargo.toml
git commit -m "Post-release change from version to path"
git push

