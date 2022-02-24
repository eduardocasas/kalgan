//! Module for the resolver which handles the tcp stream.

use crate::{
    handler::{asset, controller, middleware::Outcome},
    http::{request, request::Request, response::Response},
    settings,
    storage::cookie::Cookie,
};
use buf_redux::BufReader;
use chrono::offset::Utc;
use log::{debug, error, info, warn};
use std::{io::prelude::*, net::TcpStream};

/// Creates the `Request` object, passes it to the handlers and writes the `response` in the tcp stream.
pub fn execute(
    settings_file_path: &str,
    mut stream: TcpStream,
    controller: &fn(&Request, &str) -> Result<Response, String>,
    middleware: Option<&fn(&mut Request, &str) -> Result<Outcome, String>>,
) {
    let mut data = Vec::new();
    let mut buffer = [0; 4096];
    let mut lector = BufReader::with_capacity(10240000, &mut stream);
    loop {
        let n = lector.read(&mut buffer).unwrap();
        if n < 4096 {
            data.extend_from_slice(&buffer[..n]);
            break;
        } else {
            data.extend_from_slice(&buffer[..n]);
        }
    }
    match request::Request::new(&data) {
        Some(mut request) => {
            info!("");
            info!("Start processing new request for {}", &request.get_uri());
            stream
                .write(&get_response(
                    &mut request,
                    settings_file_path,
                    controller,
                    middleware,
                ))
                .unwrap();
            stream.flush().unwrap();
            info!("End processing request for {}", &request.get_uri());
        }
        None => {
            if data.len() == 0 {
                debug!("Empty request.");
            } else {
                warn!("The following request could not be processed:");
                warn!("{}", String::from_utf8_lossy(&data[..]).to_string());
            }
        }
    }
}
/// Returns the `response` of the handlers.
fn get_response(
    request: &mut Request,
    settings_file_path: &str,
    controller: &fn(&Request, &str) -> Result<Response, String>,
    middleware: Option<&fn(&mut Request, &str) -> Result<Outcome, String>>,
) -> Vec<u8> {
    if !settings::is_prod() && refresh_config() {
        crate::set_config(settings_file_path);
        crate::set_routes();
        #[cfg(feature = "kalgan_i18n")]
        crate::set_messages();
    }
    debug!("{:#?}", &request);
    if asset::is_static_file(&request) {
        asset::serve_static(&request)
    } else {
        let mut response = controller::resolver(request, &controller, middleware);
        match settings::get_string("cookie.renew") {
            Ok(renew) => {
                for cookie in kalgan_string::strip(&renew, ',').split(",") {
                    match settings::get_string(&format!("cookie.{}.name", cookie)) {
                        Ok(cookie_name) => {
                            if request.get_cookies().contains_key(&cookie_name) {
                                response.add_cookie(
                                    Cookie::new()
                                        .set_from_settings(cookie)
                                        .set_value(request.get_cookies()[&cookie_name].clone())
                                        .clone(),
                                );
                            }
                        }
                        Err(e) => {
                            error!("{}", e);
                            error!(
                                "Cookie {} is set to be renewed but it's not defined.",
                                cookie
                            );
                        }
                    }
                }
            }
            Err(_e) => (),
        }
        response.create()
    }
}
/// Updates `Refresh` struct with the current time.
fn refresh_config() -> bool {
    let mut refresh = crate::REFRESH.lock().unwrap();
    let old = refresh.time.clone();
    let elapsed = Utc::now().time();
    refresh.time = elapsed;
    std::mem::drop(refresh);
    (elapsed - old).num_seconds() as u64 > settings::refresh_config_timeout()
}
