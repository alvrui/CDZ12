use crate::error::EventError;
use crate::models::Event;
use serde_json::{Map, Value};
use std::collections::HashMap;

pub struct EventContext {
    pub state: Value, // Cambiado de HashMap<String, String> a Value
}

impl EventContext {
    pub fn new() -> Self {
        Self {
            state: Value::Object(Map::new()),
        }
    }

    pub fn get_path(&self, path: &str) -> Option<&Value> {
        let mut current = &self.state;
        for segment in path.split('.') {
            if let Value::Object(map) = current {
                current = map.get(segment)?;
            } else {
                return None;
            }
        }
        Some(current)
    }

    pub fn set_path(&mut self, path: &str, value: Value) -> Result<(), EventError> {
        let segments: Vec<&str> = path.split('.').collect();
        let mut current = &mut self.state;

        for segment in &segments[..segments.len() - 1] {
            if let Value::Object(map) = current {
                current = map
                    .entry(segment.to_string())
                    .or_insert_with(|| Value::Object(Map::new()));
            } else {
                return Err(EventError::InvalidPath(path.to_string()));
            }
        }

        if let Value::Object(map) = current {
            map.insert(segments.last().unwrap().to_string(), value);
        } else {
            return Err(EventError::InvalidPath(path.to_string()));
        }
        Ok(())
    }
}
