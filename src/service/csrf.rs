//! A service for CSRF protection.

use crate::http::request::Request;
use crate::service::token;
use crate::storage::session::Session;

/// Returns a random token to be used in the html form to mitigate a csrf attack.
/// # Examples
/// ```
/// use kalgan::service::csrf;
///
/// let token: String = csrf::get_token();
/// assert_eq!(token.len(), 200)
/// ```
pub fn get_token() -> String {
    token::generate(200)
}

/// Checks whether the cookie contained in the request matches the csrf session stored in redis.
pub fn is_valid(request: &Request) -> bool {
    let mut session = Session::new(&request);
    session.exists("csrf")
        && request.get_input().contains_key("csrf_token")
        && session.get("csrf") == request.get_input()["csrf_token"]
}