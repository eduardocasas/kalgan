//! Module for the controller which receives the request object and returns a response object.

use crate::{
    handler::{error, middleware, middleware::Outcome},
    http::{request::Request, response::Response},
    settings,
};
use log::info;
use std::str::FromStr;

/// Passes the `Request` to the middleware/controller linked to the route and returns the `Response` of the middleware/controller.
pub fn resolver(
    request: &mut Request,
    controller_factory: &fn(&Request, &str) -> Result<Response, String>,
    middleware_factory: Option<&fn(&mut Request, &str) -> Result<Outcome, String>>,
) -> Response {
    let routes = crate::ROUTES.lock().unwrap();
    match routes.get_route(&request.get_uri(), &request.get_method()) {
        Ok(route) => {
            info!("Route matched:");
            info!("{:#?}", &route);
            request.route = Some(route.clone());
            if request.route.as_ref().unwrap().language.is_empty()
                && settings::exists("i18n.language.default")
            {
                request.route.as_mut().unwrap().language =
                    settings::get_string("i18n.language.default").unwrap();
            }
            let controller = String::from_str(&route.get_controller()).unwrap();
            let middleware = String::from_str(&route.get_middleware()).unwrap();
            std::mem::drop(routes);
            if middleware.is_empty() {
                info!("No middleware has been defined for this route.");
                info!("Calling controller {}...", &controller);
                match controller_factory(&request, &controller) {
                    Ok(response) => response,
                    Err(e) => error::render(request, 500, &e, controller_factory),
                }
            } else {
                middleware::resolver(
                    request,
                    controller_factory,
                    middleware_factory,
                    &controller,
                    &middleware,
                )
            }
        }
        Err(e) => {
            std::mem::drop(routes);
            error::render(request, 404, &e, controller_factory)
        }
    }
}
