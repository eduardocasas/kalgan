//! A service for database connection pool management based on [sqlx crate v0.5.10](https://docs.rs/sqlx/0.5.10/sqlx/).

use crate::settings;
use log::LevelFilter;
use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPool, MySqlPoolOptions},
    postgres::{PgConnectOptions, PgPool, PgPoolOptions},
    sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions},
    ConnectOptions,
};

/// Returns the mysql connection pool based on settings parameter file.
pub async fn mysql_pool() -> MySqlPool {
    let mut options = MySqlConnectOptions::new()
        .host(&settings::get_string("db.server").unwrap())
        .username(&settings::get_string("db.user").unwrap())
        .password(&settings::get_string("db.password").unwrap())
        .database(&settings::get_string("db.name").unwrap());
    options.log_statements(LevelFilter::Debug);
    if settings::exists("db.max_connections") {
        MySqlPoolOptions::new()
            .max_connections(get_max_connections())
            .connect_with(options)
            .await
            .unwrap()
    } else {
        MySqlPoolOptions::new().connect_with(options).await.unwrap()
    }
}
/// Returns the postgresql connection pool based on settings parameter file.
pub async fn pg_pool() -> PgPool {
    let mut options = PgConnectOptions::new()
        .host(&settings::get_string("db.server").unwrap())
        .username(&settings::get_string("db.user").unwrap())
        .password(&settings::get_string("db.password").unwrap())
        .database(&settings::get_string("db.name").unwrap());
    options.log_statements(LevelFilter::Debug);
    if settings::exists("db.max_connections") {
        PgPoolOptions::new()
            .max_connections(get_max_connections())
            .connect_with(options)
            .await
            .unwrap()
    } else {
        PgPoolOptions::new().connect_with(options).await.unwrap()
    }
}
/// Returns the sqlite connection pool based on settings parameter file.
pub async fn sqlite_pool() -> SqlitePool {
    let mut options =
        SqliteConnectOptions::new().filename(&settings::get_string("db.path").unwrap());
    options.log_statements(LevelFilter::Debug);
    if settings::exists("db.max_connections") {
        SqlitePoolOptions::new()
            .max_connections(get_max_connections())
            .connect_with(options)
            .await
            .unwrap()
    } else {
        SqlitePoolOptions::new()
            .connect_with(options)
            .await
            .unwrap()
    }
}
/// Returns the number of max concurrent connections.
fn get_max_connections() -> u32 {
    settings::get_number("db.max_connections").unwrap() as u32
}
