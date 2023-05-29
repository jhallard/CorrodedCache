/*
 * This file defines the core "engine" of the key/value store. It wraps around
 * the central data structure (a HashMap), and provides the primary get/set interfaces
 */

use std::collections::HashMap;

pub struct Engine {
    data: HashMap<String, Vec<u8>>,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            data: HashMap::new(),
        }
    }

    /// Sets the value `value` for the key `key`.
    /// If the key already exists, the previous value is overwritten.
    ///
    /// # Arguments:
    /// - `key`: The key to set
    /// - `value`: The value to set
    ///
    /// # Returns
    /// Integer representing the number of bytes written for the given key
    pub fn set(&mut self, key: &str, value: Vec<u8>) -> usize {
        let len = value.len();
        self.data.insert(key.to_string(), value);
        // Return length of the value vector
        len
    }

    /// Gets the value for the key `key`.
    /// If the key does not exist, `None` is returned.
    ///
    /// # Arguments:
    /// - `key`: The key to get
    ///
    /// # Returns
    /// The value for the given key, or `None` if the key does not exist
    ///
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        self.data.get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn i64_to_bytes(i: i64) -> Vec<u8> {
        i.to_string().into_bytes()
    }

    fn bytes_to_i64(b: Vec<u8>) -> i64 {
        String::from_utf8(b).unwrap().parse::<i64>().unwrap()
    }

    #[test]
    fn test_set_and_get() {
        let mut engine = Engine::new();
        let key = "key".to_string();
        let value = vec![1, 2, 3, 4, 5];

        assert_eq!(engine.set(&key, value.clone()), value.len());
        assert_eq!(engine.get(&key), Some(value));

        let value = 1234;
        assert_eq!(engine.set(&key, i64_to_bytes(value)), 4);
        assert_eq!(bytes_to_i64(engine.get(&key).unwrap()), value);
    }

    #[test]
    fn test_get_with_no_set() {
        let engine = Engine::new();
        assert_eq!(engine.get(&"nonexistent_key"), None);
    }
}
