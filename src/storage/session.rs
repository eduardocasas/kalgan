//! A struct with a collection of methods for session management.

use crate::{http::request::Request, settings};
use kalgan_cache::{redis::ToRedisArgs, Cache};
use std::collections::HashMap;
use uuid::Uuid;

/// Main object for session management.
pub struct Session {
    cache: Cache,
    cookies: HashMap<String, String>,
}
impl Session {
    /// Creates and returns an instance of `Session` struct.
    pub fn new(request: &Request) -> Session {
        Session {
            cache: Cache::new(settings::get_string("cache.server").unwrap()),
            cookies: request.get_cookies().clone(),
        }
    }
    /// Inserts a new session record in redis and returns the session id.
    pub fn create<V: ToRedisArgs>(&mut self, value: V) -> String {
        let uuid = Uuid::new_v4();
        self.cache.insert(&uuid.to_string(), value);
        uuid.to_string()
    }
    /// Deletes a session record stored in redis.
    pub fn destroy(&mut self, session: &str) {
        let key = self.get_key(session);
        self.cache.delete(&key);
    }
    /// Checks whether a session record is stored in redis.
    pub fn exists(&mut self, session: &str) -> bool {
        self.cookies.contains_key(&self.get_cookie_key(session)) && {
            let key = self.get_key(session);
            self.cache.exists(&key)
        }
    }
    /// Updates a session record stored in redis.
    pub fn update<V: ToRedisArgs>(&mut self, session: &str, value: V) {
        let key = self.get_key(session);
        self.cache.insert(&key, value)
    }
    /// Returns a session record stored in redis.
    pub fn get(&mut self, session: &str) -> String {
        let key = self.get_key(session);
        match self.cache.get(&key) {
            Some(value) => value,
            None => "".to_string(),
        }
    }
    /// Returns the session key stored in the cookie linked to the given session.
    /// This is the session key to be used in redis to keep the session id.
    fn get_key(&mut self, session_name: &str) -> String {
        self.cookies[&self.get_cookie_key(session_name)].clone()
    }
    /// Returns the cookie name linked to the given session.
    fn get_cookie_key(&self, session_name: &str) -> String {
        settings::get_string(&format!(
            "cookie.{}.name",
            settings::get_string(&format!("session.{}.cookie", session_name)).unwrap()
        ))
        .unwrap()
    }
}
