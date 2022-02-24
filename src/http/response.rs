//! Module for the response object sent by the handler.

#[cfg(feature = "tera")]
use crate::template;
use crate::{settings, storage::cookie::Cookie};
use log::{error, warn};
#[cfg(feature = "tera")]
use tera::Context;

#[derive(Debug, Clone)]
/// The struct that contains all the data to be sent to the browser.
pub struct Response {
    status: String,
    content_type: String,
    location: String,
    cookies: Vec<Cookie>,
    content: String,
    content_length: String,
}
impl Response {
    /// Creates and returns an instance of the ´Response´ struct with all the fields empty.
    /// # Examples
    /// ```
    /// use kalgan::http::response::Response;
    ///
    /// let response: Response = Response::new();
    /// ```
    pub fn new() -> Response {
        Response {
            status: "".to_string(),
            content_type: "".to_string(),
            location: "".to_string(),
            cookies: Vec::new(),
            content: "".to_string(),
            content_length: "".to_string(),
        }
    }
    /// Sets the status field of the response and returns the instance.
    /// # Examples
    /// ```
    /// use kalgan::http::response::Response;
    /// # use kalgan::http::response::Mock;
    ///
    /// let response: Response = Response::new().set_status(200);
    ///
    /// # assert_eq!(response.mock_get_status(), "HTTP/1.1 200 OK".to_string())
    /// ```
    pub fn set_status(mut self, status_code: i32) -> Self {
        self.status = match status_code {
            200 => "HTTP/1.1 200 OK".to_string(),
            301 => "HTTP/1.1 301 Moved Permanently".to_string(),
            302 => "HTTP/1.1 302 Found".to_string(),
            403 => "HTTP/1.1 403 Forbidden".to_string(),
            404 => "HTTP/1.1 404 Not Found".to_string(),
            500 => "HTTP/1.1 500 Internal Server Error".to_string(),
            503 => "HTTP/1.1 503 Service Unavailable".to_string(),
            _ => "".to_string(),
        };
        self
    }
    /// Sets the content type field of the response and returns the instance.
    /// # Examples
    /// ```
    /// use kalgan::http::response::Response;
    /// # use kalgan::http::response::Mock;
    ///
    /// let response: Response = Response::new().set_content_type("text/html; charset=UTF-8");
    ///
    /// # assert_eq!(response.mock_get_content_type(), "\r\nContent-Type: text/html; charset=UTF-8;".to_string())
    /// ```
    pub fn set_content_type<'a>(mut self, content_type: &str) -> Self {
        self.content_type = format!("\r\nContent-Type: {};", content_type);
        self
    }
    /// Sets the location field of the response and returns the instance.
    /// # Examples
    /// ```
    /// use kalgan::http::response::Response;
    /// # use kalgan::http::response::Mock;
    ///
    /// let response: Response = Response::new().set_location("/home");
    ///
    /// # assert_eq!(response.mock_get_location(), "\r\nLocation: /home".to_string())
    /// ```
    pub fn set_location(mut self, url: &str) -> Self {
        self.location = format!("\r\nLocation: {}", url);
        self
    }
    /// Adds a cookie to the response and returns the instance.
    /// # Examples
    /// ```
    /// use kalgan::http::response::Response;
    /// use kalgan::storage::cookie::Cookie;
    /// # use kalgan::http::response::Mock;
    ///
    /// let mut response: Response = Response::new();
    /// let mut cookie: Cookie = Cookie::new();
    /// # cookie.set_name("test".to_string());
    /// response.add_cookie(cookie.clone());
    /// # assert_eq!(response.mock_get_cookies()[0], cookie)
    /// ```
    pub fn add_cookie(&mut self, cookie: Cookie) -> &Self {
        self.cookies.push(cookie);
        self
    }
    /// Adds a cookie session to the response and returns the instance.
    /// # Examples
    /// ```
    /// use kalgan::http::response::Response;
    ///
    /// let mut response: Response = Response::new().add_session("session_name", "session_key".to_string());
    /// ```
    pub fn add_session(mut self, session_name: &str, session_key_in_redis: String) -> Self {
        match settings::get_string(&format!("session.{}.cookie", session_name)) {
            Ok(cookie_name) => {
                self.add_cookie(
                    Cookie::new()
                        .set_from_settings(&cookie_name)
                        .set_value(session_key_in_redis)
                        .clone(),
                );
            }
            Err(e) => {
                warn!("{}", e);
                error!("Cookie could not be created.");
            }
        }
        self
    }
    /// Sets the content of the response and returns the instance.
    /// # Examples
    /// ```
    /// use kalgan::http::response::Response;
    /// # use kalgan::http::response::Mock;
    ///
    /// let response: Response = Response::new().set_content("<h1>Hello World</h1>");
    ///
    /// # assert_eq!(response.mock_get_content(), "<h1>Hello World</h1>")
    /// ```
    pub fn set_content(mut self, content: &str) -> Self {
        self.content = format!("{}", content);
        self
    }
    /// Sets the content length of the response and returns the instance.
    /// # Examples
    /// ```
    /// use kalgan::http::response::Response;
    /// # use kalgan::http::response::Mock;
    ///
    /// let response: Response = Response::new().set_content_length(50);
    ///
    /// # assert_eq!(response.mock_get_content_length(), "\r\nContent-Length: 50")
    /// ```
    pub fn set_content_length(mut self, content_length: usize) -> Self {
        self.content_length = format!("\r\nContent-Length: {}", content_length);
        self
    }
    /// Creates a string with all the response data to be sent to the browser.
    /// # Examples
    /// ```
    /// use kalgan::http::response::Response;
    ///
    /// let response: Vec<u8> = Response::new().create();
    /// ```
    pub fn create(&self) -> Vec<u8> {
        format!(
            "{}{}{}{}\r\n\r\n{}",
            self.status,
            self.location,
            self.create_cookies(),
            self.content_type,
            self.content
        )
        .as_bytes()
        .to_vec()
    }
    /// Creates a string with all the cookies data to be sent to the browser.
    fn create_cookies(&self) -> String {
        let mut cookie_vec: Vec<String> = Vec::new();
        for cookie in &self.cookies {
            cookie_vec.push(cookie.create());
        }
        cookie_vec.join("")
    }
}
/// Returns a json `Response` object for the given content.
/// # Examples
/// ```
/// use kalgan::http::response;
/// use kalgan::http::response::Response;
/// # use kalgan::http::response::Mock;
///
/// let response: Response = response::json("{\"name\": \"John\", \"surname\": \"Doe\"}");
/// # assert_eq!(response.mock_get_content(), "{\"name\": \"John\", \"surname\": \"Doe\"}".to_string())
/// ```
pub fn json(contents: &str) -> Response {
    Response::new()
        .set_status(200)
        .set_content_type("application/json; charset=UTF-8")
        .set_content_length(contents.len())
        .set_content(&contents)
}
/// Returns a xml `Response` object for the given content.
/// # Examples
/// ```
/// use kalgan::http::response;
/// use kalgan::http::response::Response;
/// # use kalgan::http::response::Mock;
///
/// let response: Response = response::xml("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
/// <user>\
///  <name>John</name>\
///  <surname>Doe</author>\
/// </user>");
/// # assert_eq!(response.mock_get_content(), "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
/// # <user>\
/// #   <name>John</name>\
/// #   <surname>Doe</author>\
/// # </user>".to_string())
/// ```
pub fn xml(contents: &str) -> Response {
    Response::new()
        .set_status(200)
        .set_content_type("application/xml; charset=UTF-8")
        .set_content_length(contents.len())
        .set_content(&contents)
}
/// Returns a redirect `Response` object for the given url.
/// # Examples
/// ```
/// use kalgan::http::response;
/// use kalgan::http::response::Response;
/// # use kalgan::http::response::Mock;
///
/// let response: Response = response::redirect("/home".to_string());
/// # assert_eq!(response.mock_get_location(), "\r\nLocation: /home".to_string())
/// ```
pub fn redirect(url: String) -> Response {
    Response::new().set_status(302).set_location(&url)
}
#[cfg(feature = "tera")]
/// Returns a html `Response` object for the given template.
/// # Examples
/// ```
/// use kalgan::http::response;
/// use kalgan::http::response::Response;
/// use kalgan::template::Context;
/// # use kalgan::http::response::Mock;
///
/// # kalgan::mock_settings("tests/mock/settings.yaml");
/// let response: Response = response::render("hello_world.html", Context::new());
/// # assert!(response.mock_get_content().contains("<title>Hello World :)</title>"))
/// ```
pub fn render(filename: &str, parameters: Context) -> Response {
    create_html_response(200, filename, parameters)
}
#[cfg(feature = "tera")]
/// Returns an error html `Response` object for the given error code and template.
/// # Examples
/// ```
/// use kalgan::http::response;
/// use kalgan::http::response::Response;
/// use kalgan::template::Context;
/// # use kalgan::http::response::Mock;
///
/// # kalgan::mock_settings("tests/mock/settings.yaml");
/// let response: Response = response::error(500, "error.html", Context::new());
/// # assert!(response.mock_get_content().contains("<h1>An error has occurred :(</h1>"))
/// ```
pub fn error(status: i32, filename: &str, parameters: Context) -> Response {
    create_html_response(status, filename, parameters)
}
#[cfg(feature = "tera")]
/// Returns a html `Response` object for the given status code and template.
fn create_html_response(status: i32, filename: &str, parameters: Context) -> Response {
    let contents = template::get_content(filename, parameters);
    Response::new()
        .set_status(status)
        .set_content_type("text/html; charset=UTF-8")
        .set_content_length(contents.len())
        .set_content(&contents)
}
#[cfg(feature = "test")]
/// Describes all the methods to get the `Response` fields to be used in testing.
pub trait Mock {
    /// Returns the status code of the `Response` object to be used in testing.
    fn mock_get_status(self) -> String;
    /// Returns the content type of the `Response` object to be used in testing.
    fn mock_get_content_type(self) -> String;
    /// Returns the location of the `Response` object to be used in testing.
    fn mock_get_location(self) -> String;
    /// Returns the collection of cookies of the `Response` object to be used in testing.
    fn mock_get_cookies(self) -> Vec<Cookie>;
    /// Returns the content of the `Response` object to be used in testing.
    fn mock_get_content(self) -> String;
    /// Returns the content length of the `Response` object to be used in testing.
    fn mock_get_content_length(self) -> String;
}
#[cfg(feature = "test")]
impl Mock for Response {
    fn mock_get_status(self) -> String {
        self.status
    }
    fn mock_get_content_type(self) -> String {
        self.content_type
    }
    fn mock_get_location(self) -> String {
        self.location
    }
    fn mock_get_cookies(self) -> Vec<Cookie> {
        self.cookies
    }
    fn mock_get_content(self) -> String {
        self.content
    }
    fn mock_get_content_length(self) -> String {
        self.content_length
    }
}
