use crate::error::EventError;
use crate::models::{Event, EventPool};

impl EventPool {
    pub fn add_event(&mut self, event: Event) {
        self.events.insert(event.id.id.clone(), event);
    }

    pub fn remove_event(&mut self, id: &str) -> Result<Event, EventError> {
        self.events
            .remove(id)
            .ok_or_else(|| EventError::EventNotFound(id.to_string()))
    }

    pub fn get_event(&self, id: &str) -> Result<&Event, EventError> {
        self.events
            .get(id)
            .ok_or_else(|| EventError::EventNotFound(id.to_string()))
    }

    pub fn get_event_mut(&mut self, id: &str) -> Result<&mut Event, EventError> {
        self.events
            .get_mut(id)
            .ok_or_else(|| EventError::EventNotFound(id.to_string()))
    }
}
