//! Module for the error handler.

#[cfg(feature = "tera")]
use crate::template::{self, Context, Sugar};
use crate::{
    http::{request::Request, response::Response},
    settings,
};
use log::{error, warn};

/// Returns the `Response` of the error controller (if exists).
pub fn render(
    request: &mut Request,
    error_code: i32,
    message: &str,
    controller_factory: &fn(&Request, &str) -> Result<Response, String>,
) -> Response {
    error!("{}", message);
    let controller_key = format!("error.{}", error_code);
    match &settings::get_string(&controller_key) {
        Ok(controller) => match controller_factory(&request, &controller) {
            Ok(response) => response,
            Err(e) => trigger_internal_error(request, error_code, &e),
        },
        Err(e) => {
            warn!("{}", e);
            trigger_internal_error(request, error_code, message)
        }
    }
}
/// Returns the `Response` of the internal error template.
fn trigger_internal_error(request: &Request, error_code: i32, message: &str) -> Response {
    let error_message = match error_code {
        403 => "Forbidden",
        404 => "Not Found",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "",
    };
    let contents = if settings::is_prod() {
        format!(
            " An error occurred :( <br> HTTP/1.1 {} {}.",
            error_code, error_message
        )
    } else {
        #[cfg(not(feature = "tera"))]
        {
            format!(
                "HTTP/1.1 {} {}<br><br>{}<br><br>{}.",
                error_code,
                error_message,
                message,
                request.get_raw()
            )
        }
        #[cfg(feature = "tera")]
        {
            template::get_internal_content(
                "error.html",
                Context::new()
                    .add("error_code", &error_code.to_string())
                    .add("error_message", error_message)
                    .add("message", message)
                    .add("request", request),
            )
        }
    };
    Response::new()
        .set_status(error_code)
        .set_content_type("text/html; charset=UTF-8")
        .set_content_length(contents.len())
        .set_content(&contents)
}
