//! # Kalgan - A Rust Framework for Web Developers
//!
//! Welcome to Kalgan API documentation.
//!
//! Please, see the [Official Site](https://www.markdownguide.org) for features and getting started.
#[macro_use]
extern crate lazy_static;

pub mod handler {
    mod asset;
    pub(crate) mod controller;
    mod error;
    pub mod middleware;
    pub(crate) mod resolver;
}
pub mod http {
    pub mod request;
    pub mod response;
}
pub mod service {
    #[cfg(feature = "session")]
    pub mod csrf;
    #[cfg(feature = "sqlx")]
    pub mod db;
    #[cfg(feature = "kalgan_i18n")]
    pub mod i18n;
    #[cfg(feature = "mailer")]
    pub mod mailer;
    #[cfg(feature = "services")]
    pub mod password;
    #[cfg(feature = "services")]
    pub mod token;
    pub mod url;
}
mod server {
    pub(crate) mod tcp_listener;
    pub(crate) mod thread_pool;
    pub(crate) mod worker;
}
pub mod storage {
    pub mod cookie;
    #[cfg(feature = "session")]
    pub mod session;
}
pub mod settings;
#[cfg(feature = "tera")]
pub mod template;
use crate::{
    handler::middleware::Outcome,
    http::{request::Request, response::Response},
    server::tcp_listener,
};
#[cfg(feature = "cache")]
pub use kalgan_cache;
use chrono::{offset::Utc, NaiveTime};
use log::trace;
use std::{collections::HashMap, sync::Mutex};

/// Stores the current time used to refresh the config parameters and version static files in templates.
struct Refresh {
    time: NaiveTime,
}
#[cfg(feature = "tera")]
/// Stores the custom tera configuration.
struct TeraConfig {
    config: Option<fn(&mut tera::Tera) -> &mut tera::Tera>,
}
lazy_static! {
    pub(crate) static ref CONFIG: Mutex<kalgan_config::Config> = Mutex::new(kalgan_config::Config{ collection: HashMap::new() });
    pub(crate) static ref ROUTES: Mutex<kalgan_router::Router> = Mutex::new(kalgan_router::Router{ collection: Vec::new() });
    pub(crate) static ref REFRESH: Mutex<Refresh> = Mutex::new(Refresh { time: Utc::now().time() });
}
#[cfg(feature = "tera")]
lazy_static! {
    pub(crate) static ref TEMPLATES: tera::Tera = {
        template::get_tera(&settings::get_string("tera.path").unwrap()).unwrap()
    };
    pub(crate) static ref TERA_CONFIG: Mutex<TeraConfig> = Mutex::new(TeraConfig { config: None });
}
#[cfg(feature = "kalgan_i18n")]
lazy_static! {
    pub(crate) static ref MESSAGES: Mutex<kalgan_i18n::Messages> = Mutex::new(kalgan_i18n::Messages{ collection: HashMap::new() });
}
/// Parses configuration parameters, routes and translation messages and finally starts the http server.
pub fn run(
    settings_file_path: &'static str,
    controller: fn(&Request, &str) -> Result<Response, String>,
    middleware: Option<fn(&mut Request, &str) -> Result<Outcome, String>>,
) {
    set_config(settings_file_path);
    set_routes();
    #[cfg(feature = "kalgan_i18n")]
    {
        set_messages();
    }
    tcp_listener::start(settings_file_path, controller, middleware);
}
#[cfg(feature = "tera")]
/// Sets custom tera configuration.
pub fn set_tera_config(tera_config: fn(&mut tera::Tera) -> &mut tera::Tera) {
    let mut tera_static = TERA_CONFIG.lock().unwrap();
    tera_static.config = Some(tera_config);
    std::mem::drop(tera_static);
}
/// Parses settings parameter files.
pub(crate) fn set_config(settings_file_path: &str) {
    let mut config_static = CONFIG.lock().unwrap();
    config_static.collection = kalgan_config::Config::new(&settings_file_path).collection;
    trace!("{:?}", &config_static);
    std::mem::drop(config_static);
}
/// Parses routing files.
fn set_routes() {
    let mut route_static = ROUTES.lock().unwrap();
    route_static.collection =
        kalgan_router::Router::new(&settings::get_string("router.path").unwrap()).collection;
    trace!("{:?}", &route_static);
    std::mem::drop(route_static);
}
#[cfg(feature = "kalgan_i18n")]
/// Parses translation messages files.
fn set_messages() {
    let mut messages_static = MESSAGES.lock().unwrap();
    messages_static.collection =
        kalgan_i18n::Messages::new(&settings::get_string("i18n.path").unwrap()).collection;
    trace!("{:?}", &messages_static);
    std::mem::drop(messages_static);
}
#[cfg(feature = "test")]
/// Parses settings parameter files used in testing.
pub fn mock_settings(settings_file_path: &str) {
    set_config(settings_file_path);
}
#[cfg(feature = "test")]
/// Parses routing files used in testing.
pub fn mock_routes() {
    set_routes();
}
#[cfg(feature = "test")]
/// Parses translation messages files used in testing.
pub fn mock_i18n() {
    set_messages();
}
#[cfg(test)]
pub mod tests {
    use super::*;
    pub fn set_config() {
        mock_settings("tests/mock/settings.yaml");
    }
}
