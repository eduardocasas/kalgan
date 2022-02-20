//! A service for route management.

use std::collections::HashMap;

/// Returns the uri for the given route name.
/// # Examples
/// ```yaml
/// ## routes.yaml
///
/// routes:
///   - hello_world:
///       path: /hello-world
///       controller: hello_world
///    - user:
///       path: /user/{name}/{surname}
///       controller: user_controller/index
/// ```
/// ```
/// use kalgan::service::url;
/// # use std::collections::HashMap;
///
/// # kalgan::mock_settings("tests/mock/settings.yaml");
/// # kalgan::mock_routes();
/// let hello_world_uri: String = url::generate("hello_world", HashMap::new());
/// assert_eq!(hello_world_uri, "/hello-world".to_string());
///
/// let mut parameters = HashMap::new();
/// parameters.insert("name", "john".to_string());
/// parameters.insert("surname", "doe".to_string());
/// let user_uri: String = url::generate("user", parameters);
/// assert_eq!(user_uri, "/user/john/doe".to_string());
/// ```
pub fn generate(route_name: &str, parameters: HashMap<&str, String>) -> String {
    crate::ROUTES
        .lock()
        .unwrap()
        .get_uri(route_name, parameters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        crate::tests::set_config();
        crate::mock_routes();
        assert_eq!(generate("hello_world", HashMap::new()), "/hello-world")
    }
}
