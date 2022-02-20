//! Module for the tcp listener of the http server.

use crate::{
    handler::{middleware::Outcome, resolver},
    http::{request::Request, response::Response},
    server::thread_pool::ThreadPool,
    settings,
};
use log::error;
use std::net::TcpListener;

pub(crate) fn start(
    settings_file_path: &'static str,
    controller: fn(&Request, &str) -> Result<Response, String>,
    middleware: Option<fn(&mut Request, &str) -> Result<Outcome, String>>,
) {
    match TcpListener::bind(format!(
        "{}:{}",
        settings::get_string("server.address").unwrap(),
        settings::get_number("server.port").unwrap()
    )) {
        Ok(listener) => {
            let pool = ThreadPool::new();
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let c: fn(&Request, &str) -> Result<Response, String> = controller.clone();
                let m: Option<fn(&mut Request, &str) -> Result<Outcome, String>> =
                    middleware.clone();
                pool.execute(move || {
                    resolver::execute(settings_file_path, stream, &c, m.as_ref());
                });
            }
        }
        Err(e) => {
            error!("App cannot be started.");
            error!("{}", e)
        }
    }
}
