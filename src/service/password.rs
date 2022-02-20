//! A service for password management.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Returns the argon2 hash for the given password in plain format.
/// # Examples
/// ```
/// use kalgan::service::password;
///
/// let my_password = password::hash("my_password");
/// ```
pub fn hash(plain_password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(plain_password.as_ref(), &salt)
        .unwrap()
        .to_string();
    PasswordHash::new(&password_hash).unwrap().to_string()
}
/// Checks whether the plain password matches the hashed password.
/// # Examples
/// ```
/// use kalgan::service::password;
///
/// let my_password = password::hash("my_password");
/// assert!(password::verify("my_password", my_password.as_str()));
/// ```
pub fn verify(plain_password: &str, hash_password: &str) -> bool {
    let parsed_hash = PasswordHash::new(&hash_password).unwrap();
    Argon2::default()
        .verify_password(plain_password.as_ref(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(
            &hash(&"hello_world")[..30],
            "$argon2id$v=19$m=4096,t=3,p=1$"
        )
    }
    #[test]
    fn test_verify() {
        dbg!(&hash(&"hello_world"));
        assert!(verify(&"hello_world", "$argon2id$v=19$m=4096,t=3,p=1$+rJPGukJZE2MYf+3FzyYEw$j3UQDCn7orVI3XJ/tb/MY4Kxg5zGaXj1WBZY+86LXbE"))
    }
}
