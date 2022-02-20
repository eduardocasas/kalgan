//! A service for token management.

use rand::{distributions::Alphanumeric, Rng};

/// Returns a random token with given length.
/// # Examples
/// ```
/// use kalgan::service::token;
///
/// let token: String = token::generate(50);
/// assert_eq!(token.len(), 50)
pub fn generate(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        assert_eq!(generate(50).len(), 50)
    }
}
