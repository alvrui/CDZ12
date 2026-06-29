use serde::{Deserialize, Serialize};
use serde_json::{Value, from_reader};
use std::fs::File;
use std::path::Path;
use crate::condition::Condition;
use crate::effect::Effect;
use crate::event::EventContext;
use crate::error::EventError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mechanic {
    pub id: String,
    pub condition: Condition,
    pub effects: Vec<Effect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MechanicsConfig {
    pub mechanics: Vec<Mechanic>,
}

#[derive(Debug, Clone)]
pub struct MechanicsEngine {
    mechanics: Vec<Mechanic>,
}

impl MechanicsEngine {
    pub fn new() -> Self {
        Self { mechanics: Vec::new() }
    }

    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), EventError> {
        let file = File::open(path).map_err(|e| {
            EventError::FileError(format!("Failed to open mechanics file: {}", e))
        })?;
        let config: MechanicsConfig = from_reader(file).map_err(|e| {
            EventError::FileError(format!("Failed to parse mechanics config: {}", e))
        })?;
        self.mechanics = config.mechanics;
        Ok(())
    }

    pub fn load_from_json(&mut self, json: &str) -> Result<(), EventError> {
        let config: MechanicsConfig = serde_json::from_str(json).map_err(|e| {
            EventError::FileError(format!("Failed to parse mechanics JSON: {}", e))
        })?;
        self.mechanics = config.mechanics;
        Ok(())
    }

    pub fn evaluate_and_apply(&self, context: &mut EventContext) -> Result<Vec<String>, EventError> {
        let mut triggered = Vec::new();
        for mechanic in &self.mechanics {
            if mechanic.condition.evaluate(context) {
                mechanic.apply_effects(context)?;
                triggered.push(mechanic.id.clone());
            }
        }
        Ok(triggered)
    }
}

impl Mechanic {
    pub fn apply_effects(&self, context: &mut EventContext) -> Result<(), EventError> {
        for effect in &self.effects {
            effect.apply(context)?;
        }
        Ok(())
    }
}