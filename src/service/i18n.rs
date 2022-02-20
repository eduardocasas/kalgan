//! A service for translation management.

use std::collections::HashMap;

/// Returns the message text for the given message id.
/// # Examples
/// i18n settings file:
/// ```yaml
/// ## i18n/en/messages.yaml
///
/// mock:
///     hello: Hello World
///     user: My name is {name} {surname}
/// ```
/// ```
/// use kalgan::service::i18n;
/// # use std::collections::HashMap;
///
/// # kalgan::mock_settings("tests/mock/settings.yaml");
/// # kalgan::mock_i18n();
/// let message_hello = i18n::trans("en", "mock.hello", HashMap::new());
/// assert_eq!(message_hello, "Hello World");
///
/// let mut parameters = HashMap::new();
/// parameters.insert("name", "John".to_string());
/// parameters.insert("surname", "Doe".to_string());
/// let message_user = i18n::trans("en", "mock.user", parameters);
/// assert_eq!(message_user, "My name is John Doe");
/// ```
pub fn trans(language_id: &str, message_id: &str, parameters: HashMap<&str, String>) -> String {
    crate::MESSAGES
        .lock()
        .unwrap()
        .trans(language_id, message_id, parameters)
}