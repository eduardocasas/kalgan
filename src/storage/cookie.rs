//! A struct with a collection of methods for cookie management.

use crate::settings;
use log::{debug, warn};

/// Main object for cookie management.
#[derive(Debug, Clone, PartialEq)]
pub struct Cookie {
    name: String,
    value: String,
    max_age: usize,
    expires: String,
    domain: String,
    path: String,
    secure: bool,
    http_only: bool,
}
impl Cookie {
    /// Creates and returns an instance of `Cookie` struct with the default fields.
    pub fn new() -> Cookie {
        Cookie {
            name: "Kalgan".to_string(),
            value: "...a Rust Framework for Web Developers.".to_string(),
            max_age: 24 * 60 * 60 * 1000,
            expires: "".to_string(),
            domain: "".to_string(),
            path: "/".to_string(),
            secure: true,
            http_only: true,
        }
    }
    /// Creates a string with all the cookie data to be sent to the browser.
    pub fn create(&self) -> String {
        format!(
            "\r\nSet-Cookie: {}={}; Max-Age={} {} {} {} {} {}",
            self.name,
            self.value,
            self.max_age,
            if self.expires.is_empty() {
                "".to_string()
            } else {
                format!("; Expires={}", self.max_age.to_string())
            },
            if self.domain.is_empty() {
                "".to_string()
            } else {
                format!("; Domain={}", self.domain)
            },
            if self.path.is_empty() {
                "".to_string()
            } else {
                format!("; Path={}", self.path)
            },
            if self.secure { "; Secure" } else { "" },
            if self.http_only { "; HttpOnly" } else { "" }
        )
    }
    /// Creates and returns an instance of `Cookie` struct with the fields defined in the settings file for the given cookie name.
    pub fn set_from_settings(&mut self, cookie_name: &str) -> &mut Self {
        match settings::get_string(&format!("cookie.{}.name", cookie_name)) {
            Ok(cookie_name) => {
                self.set_name(cookie_name);
            }
            Err(e) => {
                warn!("{}", e);
            }
        }
        match settings::get_string(&format!("cookie.{}.value", cookie_name)) {
            Ok(cookie_value) => {
                self.set_value(cookie_value);
            }
            Err(e) => {
                debug!("{}", e);
            }
        }
        match settings::get_number(&format!("cookie.{}.max_age", cookie_name)) {
            Ok(cookie_max_age) => {
                self.set_max_age(cookie_max_age as usize);
            }
            Err(e) => {
                debug!("{}", e);
            }
        }
        match settings::get_string(&format!("cookie.{}.expires", cookie_name)) {
            Ok(cookie_expires) => {
                self.set_expires(cookie_expires);
            }
            Err(e) => {
                debug!("{}", e);
            }
        }
        match settings::get_string(&format!("cookie.{}.domain", cookie_name)) {
            Ok(cookie_domain) => {
                self.set_domain(cookie_domain);
            }
            Err(e) => {
                debug!("{}", e);
            }
        }
        match settings::get_string(&format!("cookie.{}.path", cookie_name)) {
            Ok(cookie_path) => {
                self.set_path(cookie_path);
            }
            Err(e) => {
                debug!("{}", e);
            }
        }
        match settings::get_bool(&format!("cookie.{}.secure", cookie_name)) {
            Ok(cookie_is_secure) => {
                self.set_secure(cookie_is_secure);
            }
            Err(e) => {
                debug!("{}", e);
            }
        }
        match settings::get_bool(&format!("cookie.{}.http_only", cookie_name)) {
            Ok(cookie_is_http_only) => {
                self.set_http_only(cookie_is_http_only);
            }
            Err(e) => {
                debug!("{}", e);
            }
        }
        self
    }
    /// Sets the name of the cookie and returns the instance.
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    /// Sets the value of the cookie and returns the instance.
    pub fn set_value(&mut self, value: String) -> &mut Self {
        self.value = value;
        self
    }
    /// Sets the max age of the cookie and returns the instance.
    pub fn set_max_age(&mut self, max_age: usize) -> &mut Self {
        self.max_age = max_age;
        self
    }
    /// Sets the expire date of the cookie and returns the instance.
    pub fn set_expires(&mut self, expires: String) -> &mut Self {
        self.expires = expires;
        self
    }
    /// Sets the domain of the cookie and returns the instance.
    pub fn set_domain(&mut self, domain: String) -> &mut Self {
        self.domain = domain;
        self
    }
    /// Sets the path of the cookie and returns the instance.
    pub fn set_path(&mut self, path: String) -> &mut Self {
        self.path = path;
        self
    }
    /// Sets the http only field of the cookie and returns the instance.
    pub fn set_http_only(&mut self, http_only: bool) -> &mut Self {
        self.http_only = http_only;
        self
    }
    /// Sets the secure field of the cookie and returns the instance.
    pub fn set_secure(&mut self, secure: bool) -> &mut Self {
        self.secure = secure;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cookie = Cookie::new();
        assert_eq!(cookie.name, "Kalgan".to_string());
    }
    #[test]
    fn test_create() {
        let cookie = Cookie::new();
        assert_eq!(cookie.create(), "\r\nSet-Cookie: Kalgan=...a Rust Framework for Web Developers.; Max-Age=86400000   ; Path=/ ; Secure ; HttpOnly".to_string());
    }
    #[test]
    fn test_set_from_settings() {
        crate::tests::set_config();
        let mut cookie = Cookie::new();
        cookie.set_from_settings("mock");
        assert_eq!(cookie.name, "mock_id");
    }
    #[test]
    fn test_set_name() {
        let mut cookie = Cookie::new();
        cookie.set_name("mock_name".to_string());
        assert_eq!(cookie.name, "mock_name");
    }
    #[test]
    fn test_set_value() {
        let mut cookie = Cookie::new();
        cookie.set_value("mock_value".to_string());
        assert_eq!(cookie.value, "mock_value");
    }
    #[test]
    fn test_set_max_age() {
        let mut cookie = Cookie::new();
        cookie.set_max_age(404);
        assert_eq!(cookie.max_age, 404);
    }
    #[test]
    fn test_set_expires() {
        let mut cookie = Cookie::new();
        cookie.set_expires("mock_expires".to_string());
        assert_eq!(cookie.expires, "mock_expires".to_string());
    }
    #[test]
    fn test_set_domain() {
        let mut cookie = Cookie::new();
        cookie.set_domain("mock_domain".to_string());
        assert_eq!(cookie.domain, "mock_domain".to_string());
    }
    #[test]
    fn test_set_path() {
        let mut cookie = Cookie::new();
        cookie.set_path("/mock".to_string());
        assert_eq!(cookie.path, "/mock".to_string());
    }
    #[test]
    fn test_set_http_only() {
        let mut cookie = Cookie::new();
        cookie.set_http_only(false);
        assert_eq!(cookie.http_only, false);
    }
    #[test]
    fn test_set_secure() {
        let mut cookie = Cookie::new();
        cookie.set_secure(false);
        assert_eq!(cookie.secure, false);
    }
}
