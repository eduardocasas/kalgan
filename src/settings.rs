//! Module for system configuration parameters.

use kalgan_config::Value;
use log::warn;

const REFRESH_CONFIG_TIMEOUT: u64 = 3;
const IS_PROD: bool = false;

/// Returns the given settings parameter as `serde_yaml::Value`.
/// # Errors
/// Returns the error message.
pub fn get(key: &str) -> Result<Value, String> {
    Ok(crate::CONFIG.lock().unwrap().get(key)?)
}
/// Returns the given settings parameter as `String`.
/// # Errors
/// Returns the error message.
pub fn get_string(key: &str) -> Result<String, String> {
    Ok(crate::CONFIG.lock().unwrap().get_string(key)?)
}
/// Returns the given settings parameter as `bool`.
/// # Errors
/// Returns the error message.
pub fn get_bool(key: &str) -> Result<bool, String> {
    Ok(crate::CONFIG.lock().unwrap().get_bool(key)?)
}
/// Returns the given settings parameter as `i64`.
/// # Errors
/// Returns the error message.
pub fn get_number(key: &str) -> Result<i64, String> {
    Ok(crate::CONFIG.lock().unwrap().get_number(key)?)
}
/// Returns the given settings parameter as `f64`.
/// # Errors
/// Returns the error message.
pub fn get_float(key: &str) -> Result<f64, String> {
    Ok(crate::CONFIG.lock().unwrap().get_float(key)?)
}
/// Returns the given settings parameter as `Vec<serde_yaml::Value>`.
/// # Errors
/// Returns the error message.
pub fn get_vec(key: &str) -> Result<Vec<Value>, String> {
    Ok(crate::CONFIG.lock().unwrap().get_vec(key)?)
}
/// Checks whether the given settings parameter exists.
pub fn exists(key: &str) -> bool {
    crate::CONFIG.lock().unwrap().exists(key)
}
/// Returns the `environment.refresh_config_timeout` value.
/// If it doesn't exist it returns `REFRESH_CONFIG_TIMEOUT` const.
pub(crate) fn refresh_config_timeout() -> u64 {
    match get_number("environment.refresh_config_timeout") {
        Ok(num) => num as u64,
        Err(e) => {
            warn!("{}", e);
            warn!(
                "refresh_config_timeout is not defined. {} taken as default.",
                REFRESH_CONFIG_TIMEOUT
            );
            REFRESH_CONFIG_TIMEOUT
        }
    }
}
/// Returns the `environment.is_prod` value.
/// If it doesn't exist it returns `IS_PROD` const.
pub(crate) fn is_prod() -> bool {
    match get_bool("environment.is_prod") {
        Ok(is_prod) => is_prod,
        Err(e) => {
            warn!("{}", e);
            warn!(
                "Deployment environment is not defined. PROD = {} taken as default.",
                IS_PROD
            );
            IS_PROD
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        crate::tests::set_config();
        assert_eq!(
            get("mock.string_value").unwrap().as_str().unwrap(),
            "Hello World"
        );
    }
    #[test]
    fn test_get_string() {
        crate::tests::set_config();
        assert_eq!(get_string("mock.string_value").unwrap(), "Hello World");
    }
    #[test]
    fn test_get_bool() {
        crate::tests::set_config();
        assert!(get_bool("mock.boolean_value").unwrap())
    }
    #[test]
    fn test_get_number() {
        crate::tests::set_config();
        assert_eq!(get_number("mock.number_value").unwrap(), 1984);
    }
    #[test]
    fn test_get_float() {
        crate::tests::set_config();
        assert_eq!(get_float("mock.float_value").unwrap(), 1984.101);
    }
    #[test]
    fn test_get_vec() {
        crate::tests::set_config();
        assert_eq!(get_vec("mock.vec_value").unwrap(), ["Foo", "Bar"]);
    }
    #[test]
    fn test_exists() {
        crate::tests::set_config();
        assert!(exists("mock.boolean_value"));
    }
    #[test]
    fn test_refresh_config_timeout() {
        crate::tests::set_config();
        assert_eq!(refresh_config_timeout(), 3);
    }
    #[test]
    fn test_is_prod() {
        crate::tests::set_config();
        assert_eq!(is_prod(), false);
    }
}
