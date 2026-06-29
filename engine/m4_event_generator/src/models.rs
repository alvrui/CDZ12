use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventId {
    pub id: String,
}

impl EventId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    Simple,
    Conditional,
    Random,
    TimeBased,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub id: EventId,
    pub name: String,
    pub event_type: EventType,
    pub description: String,
    pub conditions: Vec<crate::condition::Condition>,
    pub effects: Vec<crate::effect::Effect>,
    pub weight: f32,
}

impl Event {
    pub fn new(
        id: EventId,
        name: String,
        event_type: EventType,
        description: String,
        conditions: Vec<crate::condition::Condition>,
        effects: Vec<crate::effect::Effect>,
        weight: f32,
    ) -> Self {
        Self {
            id,
            name,
            event_type,
            description,
            conditions,
            effects,
            weight,
        }
    }

    pub fn add_condition(&mut self, condition: crate::condition::Condition) {
        self.conditions.push(condition);
    }

    pub fn add_effect(&mut self, effect: crate::effect::Effect) {
        self.effects.push(effect);
    }

    pub fn evaluate_conditions(
        &self,
        context: &crate::event::EventContext,
    ) -> Result<bool, crate::error::EventError> {
        for condition in &self.conditions {
            if !condition.evaluate(context) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventPool {
    pub events: HashMap<String, Event>,
}

impl EventPool {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }

    pub fn load_from_json(json_value: Value) -> Result<Self, serde_json::Error> {
        let events_map: HashMap<String, Event> = serde_json::from_value(json_value)?;
        Ok(Self { events: events_map })
    }
}
