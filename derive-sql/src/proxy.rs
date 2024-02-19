//! Proxy to manipulate SQL queries
use super::*;

#[cfg(feature="sqlite")]
pub mod sqlite;

#[cfg(feature="mysql")]
pub mod mysql;
