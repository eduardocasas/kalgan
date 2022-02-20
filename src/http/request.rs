//! Module for the request object passed to the handler.

use log::warn;
use kalgan_router::Route;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use urlencoding::decode;

#[derive(Serialize, Deserialize, Debug, Clone)]
/// The struct that contains all the data of the file attached to the request.
pub struct File<'a> {
    pub filename: String,
    pub content_type: String,
    pub content: &'a [u8],
}
#[derive(Serialize, Deserialize, Debug, Clone)]
/// The struct that contains all the data sent by the browser.
pub struct Request<'a> {
    method: String,
    uri: String,
    protocol: String,
    cookies: HashMap<String, String>,
    host: String,
    user_agent: String,
    input: HashMap<String, String>,
    referer: String,
    #[serde(borrow)]
    files: HashMap<String, File<'a>>,
    raw: String,
    pub middleware: HashMap<String, String>,
    pub route: Option<Route>,
}
impl<'a> Request<'a> {
    /// Creates and returns an instance of the ´Request´ struct with the data sent by the browser.
    pub fn new(buffer: &[u8]) -> Option<Request> {
        let raw = String::from_utf8_lossy(&buffer[..]).to_string();
        let first_line = raw.split("\r\n").next()?;
        let mut parameters = first_line.split(" ");
        Some(Request {
            method: parameters.next()?.to_string(),
            uri: Request::parse_url_encoding(&parameters.next()?.trim().to_string()),
            protocol: parameters.next()?.to_string(),
            cookies: Request::parse_cookies(&raw),
            host: Request::parse_host(&raw),
            user_agent: Request::parse_user_agent(&raw),
            input: Request::parse_input(&raw),
            referer: Request::parse_referer(&raw),
            files: Request::parse_files(&raw, buffer),
            raw: raw,
            middleware: HashMap::new(),
            route: None,
        })
    }
    /// Returns the http method of the request.
    /// # Examples
    /// ```
    /// use kalgan::http::request::Request;
    /// # use kalgan::http::request::Mock;
    ///
    /// # let request = Request::mock().mock_set_method("POST".to_string());
    /// let method: &String = request.get_method();
    /// # assert_eq!(method, &"POST".to_string())
    /// ```
    pub fn get_method(&self) -> &String {
        &self.method
    }
    /// Returns the uri of the request.
    /// # Examples
    /// ```
    /// use kalgan::http::request::Request;
    /// # use kalgan::http::request::Mock;
    ///
    /// # let request = Request::mock().mock_set_uri("/home".to_string());
    /// let uri: &String = request.get_uri();
    /// # assert_eq!(uri, &"/home".to_string())
    /// ```
    pub fn get_uri(&self) -> &String {
        &self.uri
    }
    /// Returns the http protocol of the request.
    /// # Examples
    /// ```
    /// use kalgan::http::request::Request;
    /// # use kalgan::http::request::Mock;
    ///
    /// # let request = Request::mock().mock_set_protocol("HTTP/1.1".to_string());
    /// let protocol: &String = request.get_protocol();
    /// # assert_eq!(protocol, &"HTTP/1.1".to_string())
    /// ```
    pub fn get_protocol(&self) -> &String {
        &self.protocol
    }
    /// Returns the collection of cookies of the request.
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use kalgan::http::request::Request;
    /// # use kalgan::http::request::Mock;
    /// # let mut cookies_right = HashMap::new();
    /// # cookies_right.insert("key".to_string(), "value".to_string());
    /// # let request = Request::mock().mock_set_cookies(cookies_right.clone());
    /// let cookies: &HashMap<String, String> = request.get_cookies();
    /// # assert_eq!(cookies["key"], cookies_right["key"])
    /// ```
    pub fn get_cookies(&self) -> &HashMap<String, String> {
        &self.cookies
    }
    /// Returns the host field of the request.
    /// # Examples
    /// ```
    /// use kalgan::http::request::Request;
    /// # use kalgan::http::request::Mock;
    ///
    /// # let request = Request::mock().mock_set_host("dev.foobar.com".to_string());
    /// let host: &String = request.get_host();
    /// # assert_eq!(host, &"dev.foobar.com".to_string())
    /// ```
    pub fn get_host(&self) -> &String {
        &self.host
    }
    /// Returns the user agent of the request.
    /// # Examples
    /// ```
    /// use kalgan::http::request::Request;
    /// # use kalgan::http::request::Mock;
    ///
    /// # let request = Request::mock().mock_set_user_agent("Mozilla/5.0".to_string());
    /// let user_agent: &String = request.get_user_agent();
    /// # assert_eq!(user_agent, &"Mozilla/5.0".to_string())
    /// ```
    pub fn get_user_agent(&self) -> &String {
        &self.user_agent
    }
    /// Returns the input data collection of the request.
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use kalgan::http::request::Request;
    /// # use kalgan::http::request::Mock;
    ///
    /// # let mut input_right = HashMap::new();
    /// # input_right.insert("key".to_string(), "value".to_string());
    /// # let request = Request::mock().mock_set_input(input_right.clone());
    /// let input: &HashMap<String, String> = request.get_input();
    /// # assert_eq!(input["key"], input_right["key"])
    /// ```
    pub fn get_input(&self) -> &HashMap<String, String> {
        &self.input
    }
    /// Returns the referer field of the request.
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use kalgan::http::request::Request;
    /// # use kalgan::http::request::Mock;
    ///
    /// # let request = Request::mock().mock_set_referer("/home".to_string());
    /// let referer: &String = request.get_referer();
    /// # assert_eq!(referer, &"/home".to_string());
    /// ```
    pub fn get_referer(&self) -> &String {
        &self.referer
    }
    /// Returns the collection of files attached to the request.
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use kalgan::http::request::Request;
    /// use kalgan::http::request::File;
    /// # use kalgan::http::request::Mock;
    ///
    /// # let mut files_right = HashMap::new();
    /// # files_right.insert("key".to_string(), File {
    /// #     filename: "test".to_string(),
    /// #     content_type: "text/png".to_string(),
    /// #     content: &[0, 0, 0, 1]
    /// # });
    /// # let request = Request::mock().mock_set_files(files_right.clone());
    /// let files: &HashMap<String, File> = request.get_files();
    /// # assert_eq!(files["key"].filename, files_right["key"].filename);
    /// ```
    pub fn get_files(&self) -> &HashMap<String, File<'a>> {
        &self.files
    }
    /// Returns the raw value of the request.
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use kalgan::http::request::Request;
    /// # use kalgan::http::request::Mock;
    ///
    /// # let request = Request::mock().mock_set_raw("foobar".to_string());
    /// let raw: &String = request.get_raw();
    /// # assert_eq!(raw, &"foobar".to_string());
    /// ```
    pub fn get_raw(&self) -> &String {
        &self.raw
    }
    /// Parses and returns the collection of cookies of the request.
    fn parse_cookies(request: &str) -> HashMap<String, String> {
        let mut cookies: HashMap<String, String> = HashMap::new();
        let cookies_collection = match regex::Regex::new(r#"Cookie:.*"#).unwrap().find(request) {
            Some(x) => x
                .as_str()
                .to_string()
                .replace("Cookie:", "")
                .replace(" ", "")
                .to_string(),
            None => "".to_string(),
        };
        let collection: Vec<&str> = cookies_collection.split(";").collect();
        for cookie in collection {
            if cookie.contains("=") {
                let pos = cookie.find("=").unwrap();
                let re = regex::Regex::new(r"\s+").unwrap();
                let cookie_without_spaces = re.replace_all(&cookie, " ").trim().to_string();
                cookies.insert(
                    cookie_without_spaces[..pos].to_string(),
                    cookie_without_spaces[pos + 1..].to_string(),
                );
            }
        }
        cookies
    }
    /// Parses and returns the host field of the request.
    fn parse_host(request: &str) -> String {
        match regex::Regex::new(r#"Host:.*"#).unwrap().find(request) {
            Some(x) => {
                let mut string = x.as_str().replace("Host:", "").replace(" ", "").to_string();
                let len = string.trim_end_matches(&['\r', '\n'][..]).len();
                string.truncate(len);
                Request::parse_url_encoding(&string)
            }
            None => "".to_string(),
        }
    }
    /// Parses and returns the user agent of the request.
    fn parse_user_agent(request: &str) -> String {
        match regex::Regex::new(r#"User-Agent:.*"#).unwrap().find(request) {
            Some(x) => {
                let mut string = x
                    .as_str()
                    .replace("User-Agent:", "")
                    .replace(" ", "")
                    .to_string();
                let len = string.trim_end_matches(&['\r', '\n'][..]).len();
                string.truncate(len);
                string
            }
            None => "".to_string(),
        }
    }
    /// Parses and returns the input data collection of the request.
    fn parse_input(request: &str) -> HashMap<String, String> {
        let mut input: HashMap<String, String> = HashMap::new();
        match regex::Regex::new(r#"Content-Type: multipart/form-data;"#)
            .unwrap()
            .find(request)
        {
            Some(_x) => {
                for mat in regex::Regex::new(r#"Content-Disposition: form-data; name=.*"#)
                    .unwrap()
                    .find_iter(request)
                {
                    let mut key = mat
                        .as_str()
                        .replace("Content-Disposition: form-data; name=", "")
                        .replace(" ", "")
                        .replace("\"", "")
                        .to_string();
                    let len = key.trim_end_matches(&['\r', '\n'][..]).len();
                    key.truncate(len);
                    if !key.contains("filename=") {
                        let request_clone = request[mat.end()..].to_string().clone();
                        let lines: Vec<&str> = request_clone.split("\r\n").collect();
                        input.insert(
                            Request::parse_url_encoding(&key),
                            Request::parse_url_encoding(&lines[1].to_string()),
                        );
                    }
                }
            }
            None => {
                let lines: Vec<&str> = request.split("\r\n").collect();
                let len = lines.len();
                if lines[len - 2].trim() == "" {
                    let parameters: Vec<&str> = lines[len - 1].split("&").collect();
                    for parameter in parameters {
                        if parameter.contains("=") {
                            let param: Vec<&str> = parameter.split("=").collect();
                            input.insert(
                                Request::parse_url_encoding(&param[0].to_string()),
                                Request::parse_url_encoding(
                                    &param[1].to_string().replace("+", " "),
                                ),
                            );
                        }
                    }
                }
            }
        }
        input
    }
    /// Parses and returns the referer field of the request.
    fn parse_referer(request: &str) -> String {
        match regex::Regex::new(r#"Referer:.*"#).unwrap().find(request) {
            Some(x) => {
                let mut string = x
                    .as_str()
                    .to_string()
                    .replace("Referer:", "")
                    .replace(" ", "")
                    .to_string();
                let len = string.trim_end_matches(&['\r', '\n'][..]).len();
                string.truncate(len);
                string
            }
            None => "".to_string(),
        }
    }
    /// Parses and returns the uri of the request.
    fn parse_url_encoding(url_encoded_string: &String) -> String {
        match decode(&url_encoded_string) {
            Ok(s) => s.to_string(),
            Err(e) => {
                warn!("{}", e);
                url_encoded_string.to_string()
            }
        }
    }
    /// Parses and returns the collection of files attached to the request.
    fn parse_files<'b>(request: &str, buffer: &'b [u8]) -> HashMap<String, File<'b>> {
        let mut files: HashMap<String, File> = HashMap::new();
        match regex::Regex::new(r#"Content-Type: multipart/form-data;.*?\r\n"#)
            .unwrap()
            .find(request)
        {
            Some(x) => {
                let param: Vec<&str> = request[x.start()..x.end()]
                    .trim_end_matches(&['\r', '\n'][..])
                    .split("; boundary=")
                    .collect();
                let boundary = &param[1];
                let mut prev = 0;
                for mat in regex::Regex::new(
                    format!(
                        r#"Content-Disposition: form-data; name=(?s:.*?){}"#,
                        boundary
                    )
                    .as_str(),
                )
                .unwrap()
                .find_iter(request)
                {
                    let request_clone = request[mat.start()..mat.end()].to_string().clone();
                    let lines: Vec<&str> = request_clone
                        .trim_start_matches(&['\r', '\n'][..])
                        .split("\r\n")
                        .collect();
                    let first_line = lines[0].to_string();
                    if first_line.contains("filename=") && !first_line.contains("filename=\"\"") {
                        let params: Vec<&str> = first_line.split("filename=").collect();
                        let mut content_type = lines[1]
                            .to_string()
                            .replace("Content-Type:", "")
                            .replace(" ", "")
                            .replace("\"", "")
                            .to_string();
                        let len = content_type.trim_end_matches(&['\r', '\n'][..]).len();
                        content_type.truncate(len);
                        let x1 = request_clone.find(&lines[3].to_string()).unwrap();
                        let x2 = request_clone
                            .find(&lines[lines.len() - 1].to_string())
                            .unwrap();
                        let content = request_clone[x1..x2].trim_end_matches(&['\r', '\n'][..]);
                        let n1 = if prev == 0 {
                            request.to_string().find(content).unwrap()
                        } else {
                            let mut new_n1 = prev.clone();
                            loop {
                                if String::from_utf8_lossy(&buffer[new_n1..new_n1 + 4])
                                    .to_string()
                                    .find("\r\n\r\n")
                                    == Some(0)
                                {
                                    break new_n1 + 4;
                                }
                                new_n1 = new_n1 + 1;
                            }
                        };
                        let mut n2 = n1.clone();
                        loop {
                            n2 = n2 + 1;
                            let test = String::from_utf8_lossy(&buffer[n2 - boundary.len()..n2])
                                .to_string();
                            if test.contains(boundary) {
                                prev = n2.clone();
                                n2 = n2 - 4 - boundary.len();
                                break;
                            }
                        }
                        files.insert(
                            kalgan_string::strip(
                                kalgan_string::strip_right(
                                    params[0]
                                        .replace("Content-Disposition: form-data; name=", "")
                                        .trim(),
                                    ';',
                                ),
                                '"',
                            )
                            .to_string(),
                            File {
                                filename: params[1].replace(" ", "").replace("\"", ""),
                                content_type: content_type,
                                content: &buffer[n1..n2],
                            },
                        );
                    }
                }
            }
            None => (),
        }
        files
    }
}
#[cfg(feature = "test")]
/// Describes all the methods to set the `Request` fields to be used in testing.
pub trait Mock<'a> {
    /// Creates the `Request` object with empty fields to be used in testing and returns the instance.
    fn mock() -> Self;
    /// Sets the http method of the request and returns the instance.
    fn mock_set_method(self, method: String) -> Self;
    /// Sets the uri of the request and returns the instance.
    fn mock_set_uri(self, uri: String) -> Self;
    /// Sets the protocol of the request and returns the instance.
    fn mock_set_protocol(self, protocol: String) -> Self;
    /// Sets the collection of cookies of the request and returns the instance.
    fn mock_set_cookies(self, cookies: HashMap<String, String>) -> Self;
    /// Sets the host field of the request and returns the instance.
    fn mock_set_host(self, host: String) -> Self;
    /// Sets the user agent of the request and returns the instance.
    fn mock_set_user_agent(self, user_agent: String) -> Self;
    /// Sets the input data collection of the request and returns the instance.
    fn mock_set_input(self, input: HashMap<String, String>) -> Self;
    /// Sets the referer field the request and returns the instance.
    fn mock_set_referer(self, referer: String) -> Self;
    /// Sets the collection of files attached to the request and returns the instance.
    fn mock_set_files(self, files: HashMap<String, File<'a>>) -> Self;
    /// Sets the raw value of the request and returns the instance.
    fn mock_set_raw(self, raw: String) -> Self;
}
#[cfg(feature = "test")]
impl<'a> Mock<'a> for Request<'a> {
    fn mock() -> Self {
        Request {
            method: "".to_string(),
            uri: "".to_string(),
            protocol: "".to_string(),
            cookies: HashMap::new(),
            host: "".to_string(),
            user_agent: "".to_string(),
            input: HashMap::new(),
            referer: "".to_string(),
            files: HashMap::new(),
            raw: "".to_string(),
            middleware: HashMap::new(),
            route: None,
        }
    }
    fn mock_set_method(mut self, method: String) -> Self {
        self.method = method;
        self
    }
    fn mock_set_uri(mut self, uri: String) -> Self {
        self.uri = uri;
        self
    }
    fn mock_set_protocol(mut self, protocol: String) -> Self {
        self.protocol = protocol;
        self
    }
    fn mock_set_cookies(mut self, cookies: HashMap<String, String>) -> Self {
        self.cookies = cookies;
        self
    }
    fn mock_set_host(mut self, host: String) -> Self {
        self.host = host;
        self
    }
    fn mock_set_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }
    fn mock_set_input(mut self, input: HashMap<String, String>) -> Self {
        self.input = input;
        self
    }
    fn mock_set_referer(mut self, referer: String) -> Self {
        self.referer = referer;
        self
    }
    fn mock_set_files(mut self, files: HashMap<String, File<'a>>) -> Self {
        self.files = files;
        self
    }
    fn mock_set_raw(mut self, raw: String) -> Self {
        self.raw = raw;
        self
    }
}
