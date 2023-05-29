/*
 * This file defines the core "engine" of the key/value store. It wraps around
 * the central data structure (a HashMap), and provides the primary interfaces:
 * - `get(key: String) -> Option<String>`
 * - `set(key: String, value: String) -> Option<String>`
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

    #[test]
    fn test_set_and_get() {
        let mut engine = Engine::new();
        let key = "key".to_string();
        let value = vec![1, 2, 3, 4, 5];

        engine.set(&key, value.clone());
        assert_eq!(engine.get(&key), Some(value));
    }

    #[test]
    fn test_get_with_no_set() {
        let engine = Engine::new();
        assert_eq!(engine.get(&"nonexistent_key"), None);
    }
}
