//! Module for template system based on [tera crate v1.15.0](https://docs.rs/tera/1.15.0/tera/).

use crate::settings;
use log::{debug, error, info, warn};
use serde::Serialize;
use std::{collections::HashMap, str::FromStr};
pub use tera::{Context as TeraContext, Result, Tera, Value};

/// A type alias for `tera::Context`.
pub type Context = TeraContext;

/// Adds syntactic sugar for `tera::Context` object.
pub trait Sugar {
    /// This method inserts a new *value* in `tera::Context` object and returns this object.
    /// # Examples
    /// ```
    /// use kalgan::template::Sugar;
    /// use kalgan::template::Context;
    ///
    /// let context = Context::new()
    ///     .add("key_str", "Hello World!")
    ///     .add("key_string", &"Hello World!".to_string())
    ///     .add("key_num", &101);
    ///
    /// assert_eq!("Hello World!", context.get("key_str").unwrap())
    /// ```
    fn add<T: Serialize + ?Sized, S: Into<String>>(&mut self, key: S, val: &T) -> Self;
}

impl Sugar for Context {
    fn add<T: Serialize + ?Sized, S: Into<String>>(&mut self, key: S, val: &T) -> Self {
        self.insert(key, val);
        self.clone()
    }
}

/// Returns the content of a template parsed by `tera`.
/// # Errors
/// Returns the content of the error message.
/// If running in a production environment a simple Error 500 will be displayed.
pub(crate) fn get_content(filename: &str, parameters: Context) -> String {
    if settings::is_prod() {
        match &crate::TEMPLATES.render(filename, &parameters) {
            Ok(content) => {
                info!("Rendering Template {}...", &filename);
                String::from_str(content).unwrap()
            }
            Err(e) => {
                warn!("{}", e);
                " An error occurred :( <br> HTTP/1.1 500 Internal Server Error.".to_string()
            }
        }
    } else {
        match get_tera(&settings::get_string("tera.path").unwrap()) {
            Ok(tera) => match tera.render(filename, &parameters) {
                Ok(content) => {
                    info!("Rendering Template {}...", &filename);
                    content
                }
                Err(e) => {
                    warn!("{}", e);
                    e.to_string()
                }
            },
            Err(e) => {
                error!("{}", e);
                format!("<h1>500 Internal Server Error</h1><p>The following error has been reported by Tera:</p><br><pre>{}</pre>", e.to_string())
            }
        }
    }
}
/// Returns the content of the internal template error parsed by `tera`.
/// # Errors
/// Returns the content of the error message.
pub(crate) fn get_internal_content(filename: &str, parameters: Context) -> String {
    match get_tera(&format!("{}/template", env!("CARGO_MANIFEST_DIR"))) {
        Ok(tera) => match tera.render(filename, &parameters) {
            Ok(content) => {
                info!("Rendering Template {}...", &filename);
                content
            }
            Err(e) => {
                warn!("{}", e);
                e.to_string()
            }
        },
        Err(e) => {
            error!("{}", e);
            format!("<h1>500 Internal Server Error</h1><p>The following error has been reported by Tera:</p><br><pre>{}</pre>", e.to_string())
        }
    }
}
/// Returns a new `Tera` instance.
pub(crate) fn get_tera(templates_folder: &str) -> Result<Tera> {
    let mut tera = Tera::new(&format!("{}/**/*", templates_folder))?;
    debug!("{:?}", &tera);
    #[cfg(feature = "kalgan_i18n")]
    {
        tera.register_filter("trans", trans);
    }
    tera.register_filter("url", url);
    tera.register_filter("asset", asset);
    let tera_config = crate::TERA_CONFIG.lock().unwrap();
    match tera_config.config {
        Some(config) => {
            config(&mut tera);
        }
        None => (),
    }
    std::mem::drop(tera_config);
    Ok(tera)
}
/// A custom filter for `tera` for message translation.
#[cfg(feature = "kalgan_i18n")]
fn trans(value: &Value, parameters: &HashMap<String, Value>) -> Result<Value> {
    let mut collection: HashMap<&str, String> = HashMap::new();
    let mut language: String = "".to_string();
    for (key, val) in parameters.into_iter() {
        if key.as_str() == "_lang" {
            language = kalgan_string::strip(&val.to_string(), '"').to_string();
        } else {
            collection.insert(
                key.as_str(),
                kalgan_string::strip(&val.to_string(), '"').to_string(),
            );
        }
    }
    if language.is_empty() {
        let default = settings::get_string("i18n.language.default").unwrap();
        debug!("Language not specified: \"{}\" taken by default.", default);
        Ok(Value::String(crate::MESSAGES.lock().unwrap().trans(
            &default,
            kalgan_string::strip(&value.to_string(), '"'),
            collection,
        )))
    } else {
        Ok(Value::String(crate::MESSAGES.lock().unwrap().trans(
            &language,
            kalgan_string::strip(&value.to_string(), '"'),
            collection,
        )))
    }
}
/// A custom filter for `tera` for uri generation.
fn url(value: &Value, parameters: &HashMap<String, Value>) -> Result<Value> {
    let mut collection: HashMap<&str, String> = HashMap::new();
    for (key, val) in parameters.into_iter() {
        collection.insert(
            key.as_str(),
            kalgan_string::strip(&val.to_string(), '"').to_string(),
        );
    }
    Ok(Value::String(crate::ROUTES.lock().unwrap().get_uri(
        kalgan_string::strip(&value.to_string(), '"'),
        collection,
    )))
}
/// A custom filter for `tera` to call static files.
fn asset(value: &Value, _parameters: &HashMap<String, Value>) -> Result<Value> {
    let static_version = if settings::is_prod() && settings::exists("static.version") {
        settings::get_string("static.version").unwrap()
    } else {
        let refresh = crate::REFRESH.lock().unwrap();
        let datetime = refresh.time.clone();
        std::mem::drop(refresh);
        datetime.to_string()
    };
    let path = match settings::get_string("static.path") {
        Ok(path) => path,
        Err(_e) => "".to_string(),
    };
    Ok(Value::String(format!(
        "{}{}?{}",
        path,
        kalgan_string::strip(&value.to_string(), '"'),
        static_version
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::request::Mock;
    use crate::http::request::Request;

    #[test]
    fn test_get_content() {
        crate::tests::set_config();
        let content = get_content("hello_world.html", Context::new());
        assert!(content.contains("<h1>Hello World :)</h1>"))
    }
    #[test]
    fn test_get_internal_content() {
        crate::tests::set_config();
        let content = get_internal_content(
            "error.html",
            Context::new()
                .add("error_code", &500)
                .add("error_message", "Internal Server Error")
                .add("message", "This is a test.")
                .add("request", &Request::mock()),
        );
        assert!(content.contains("Error 500 | Internal Server Error | Kalgan Framework"))
    }
}
