use crate::error::EventError;
use crate::event::EventContext;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Effect {
    pub action: String,
    pub target: String,
    pub value: Option<Value>,
    pub delta: Option<i32>,
}

impl Effect {
    pub fn apply(&self, context: &mut EventContext) -> Result<(), EventError> {
        match self.action.as_str() {
            "set_flag" => {
                let value = self
                    .value
                    .as_ref()
                    .and_then(|v| v.as_bool())
                    .ok_or_else(|| {
                        EventError::EffectApplicationError(
                            "set_flag requires a boolean value".to_string(),
                        )
                    })?;
                context.set_path(&self.target, Value::Bool(value))?;
                Ok(())
            }

            "modify_medidor" => {
                let delta = self.delta.ok_or_else(|| {
                    EventError::EffectApplicationError(
                        "modify_medidor requires a delta value".to_string(),
                    )
                })?;

                if let Some(current) = context.get_path(&self.target) {
                    if let Some(current_val) = current.as_i64() {
                        let new_val = (current_val as i32) + delta;
                        context.set_path(&self.target, Value::from(new_val))?;
                        return Ok(());
                    }
                }

                context.set_path(&self.target, Value::from(delta))?;
                Ok(())
            }

            _ => Err(EventError::EffectApplicationError(format!(
                "Unknown effect action: {}",
                self.action
            ))),
        }
    }
}

impl<'de> Deserialize<'de> for Effect {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let obj = value
            .as_object()
            .ok_or_else(|| serde::de::Error::custom("Effect must be an object"))?;

        let action = obj
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| serde::de::Error::custom("Effect missing action field"))?
            .to_string();

        let target = obj
            .get("target")
            .and_then(|v| v.as_str())
            .ok_or_else(|| serde::de::Error::custom("Effect missing target field"))?
            .to_string();

        let value = obj.get("value").cloned();
        let delta = obj.get("delta").and_then(|v| v.as_i64()).map(|d| d as i32);

        Ok(Effect {
            action,
            target,
            value,
            delta,
        })
    }
}
