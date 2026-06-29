use crate::event::EventContext;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Condition {
    AlwaysTrue,
    StateEquals { key: String, value: String },
    StateGreaterThan { key: String, threshold: i32 },
    StateLessThan { key: String, threshold: i32 },
    StateIn { key: String, values: Vec<String> },
    And { conditions: Vec<Condition> },
    Or { conditions: Vec<Condition> },
    Not { condition: Box<Condition> },
}

impl Condition {
    pub fn evaluate(&self, context: &EventContext) -> bool {
        match self {
            Condition::AlwaysTrue => true,

            Condition::StateEquals { key, value } => {
                if let Some(actual) = context.get_path(key) {
                    if let Some(actual_str) = actual.as_str() {
                        return actual_str == value;
                    }
                }
                false
            }

            Condition::StateGreaterThan { key, threshold } => {
                if let Some(actual) = context.get_path(key) {
                    if let Some(actual_val) = actual.as_i64() {
                        return actual_val > (*threshold as i64);
                    }
                }
                false
            }

            Condition::StateLessThan { key, threshold } => {
                if let Some(actual) = context.get_path(key) {
                    if let Some(actual_val) = actual.as_i64() {
                        return actual_val < (*threshold as i64);
                    }
                }
                false
            }

            Condition::StateIn { key, values } => {
                if let Some(actual) = context.get_path(key) {
                    if let Some(actual_str) = actual.as_str() {
                        return values.contains(&actual_str.to_string());
                    }
                }
                false
            }

            Condition::And { conditions } => conditions.iter().all(|c| c.evaluate(context)),

            Condition::Or { conditions } => conditions.iter().any(|c| c.evaluate(context)),

            Condition::Not { condition } => !condition.evaluate(context),
        }
    }
}

impl<'de> Deserialize<'de> for Condition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        if let Value::Object(map) = &value {
            if map.contains_key("AND") {
                let conditions: Vec<Condition> = serde_json::from_value(map["AND"].clone())
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?;
                return Ok(Condition::And { conditions });
            }

            if map.contains_key("OR") {
                let conditions: Vec<Condition> = serde_json::from_value(map["OR"].clone())
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?;
                return Ok(Condition::Or { conditions });
            }

            if map.contains_key("NOT") {
                let condition: Condition = serde_json::from_value(map["NOT"].clone())
                    .map_err(|e| serde::de::Error::custom(e.to_string()))?;
                return Ok(Condition::Not {
                    condition: Box::new(condition),
                });
            }

            if map.contains_key("StateEquals") {
                let data = map["StateEquals"]
                    .as_object()
                    .ok_or_else(|| serde::de::Error::custom("StateEquals must be an object"))?;
                let key = data
                    .get("key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| serde::de::Error::custom("StateEquals missing key"))?
                    .to_string();
                let value = data
                    .get("value")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| serde::de::Error::custom("StateEquals missing value"))?
                    .to_string();
                return Ok(Condition::StateEquals { key, value });
            }

            if map.contains_key("StateGreaterThan") {
                let data = map["StateGreaterThan"].as_object().ok_or_else(|| {
                    serde::de::Error::custom("StateGreaterThan must be an object")
                })?;
                let key = data
                    .get("key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| serde::de::Error::custom("StateGreaterThan missing key"))?
                    .to_string();
                let threshold = data
                    .get("threshold")
                    .and_then(|v| v.as_i64())
                    .ok_or_else(|| serde::de::Error::custom("StateGreaterThan missing threshold"))?
                    as i32;
                return Ok(Condition::StateGreaterThan { key, threshold });
            }

            if map.contains_key("StateLessThan") {
                let data = map["StateLessThan"]
                    .as_object()
                    .ok_or_else(|| serde::de::Error::custom("StateLessThan must be an object"))?;
                let key = data
                    .get("key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| serde::de::Error::custom("StateLessThan missing key"))?
                    .to_string();
                let threshold = data
                    .get("threshold")
                    .and_then(|v| v.as_i64())
                    .ok_or_else(|| serde::de::Error::custom("StateLessThan missing threshold"))?
                    as i32;
                return Ok(Condition::StateLessThan { key, threshold });
            }

            if map.contains_key("StateIn") {
                let data = map["StateIn"]
                    .as_object()
                    .ok_or_else(|| serde::de::Error::custom("StateIn must be an object"))?;
                let key = data
                    .get("key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| serde::de::Error::custom("StateIn missing key"))?
                    .to_string();
                let values = data
                    .get("values")
                    .and_then(|v| v.as_array())
                    .ok_or_else(|| serde::de::Error::custom("StateIn missing values"))?
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();
                return Ok(Condition::StateIn { key, values });
            }
        }

        if let Value::String(s) = &value {
            if s == "always_true" {
                return Ok(Condition::AlwaysTrue);
            }
        }

        Err(serde::de::Error::custom(format!(
            "Invalid condition format: {:?}",
            value
        )))
    }
}
