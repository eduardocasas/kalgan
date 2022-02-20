//! Module for the middleware which receives the request object and returns the outcome object.

use crate::{
    handler::error,
    http::{request::Request, response::Response},
};
use log::info;

#[derive(Debug)]
/// The object returned by the middleware.
/// * If `Outcome.success` is set to `true` the `Request` is passed to the controller.
/// * If `Outcome.success` is set to `false` the controller is skipped and the `Outcome.response` is sent to the browser.
pub struct Outcome {
    pub success: bool,
    pub response: Option<Response>,
}
/// Passes the `Request` to the middleware/controller linked to the route and returns the `Response` of the middleware/controller.
pub(crate) fn resolver(
    request: &mut Request,
    controller_factory: &fn(&Request, &str) -> Result<Response, String>,
    middleware_factory: Option<&fn(&mut Request, &str) -> Result<Outcome, String>>,
    controller: &str,
    middleware: &str,
) -> Response {
    match middleware_factory {
        Some(factory) => {
            info!("Calling middleware {}...", &middleware);
            match factory(request, &middleware) {
                Ok(outcome) => {
                    if outcome.success {
                        info!("Middleware response is successful.");
                        info!("Calling controller {}...", &controller);
                        match controller_factory(&request, &controller) {
                            Ok(response) => response,
                            Err(e) => error::render(request, 500, &e, controller_factory)
                        }
                    } else {
                        info!("Middleware response is not successful.");
                        info!("Controller {} is skipped.", &controller);
                        match outcome.response {
                            Some(response) => response,
                            None => error::render(request, 500, "No Middleware response was set.", controller_factory)
                        }
                    }
                }, Err(e) => error::render(request, 500, &e, controller_factory)
            }
        }, None => error::render(request, 500, &format!("Middleware \"{}\" is set for Controller \"{}\" but Middleware Factory has not been declared.", &middleware, &controller), controller_factory)
    }
}
