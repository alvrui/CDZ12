use m4_event_generator::{Condition, Effect, Event, EventId, EventType};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_create_event() {
        let mut conditions = Vec::new();
        conditions.push(Condition::new(
            "always_true".to_string(),
            HashMap::new(),
        ));

        let mut effects = Vec::new();
        effects.push(Effect::new(
            "set_state".to_string(),
            [("key".to_string(), "value".to_string())]
                .iter()
                .cloned()
                .collect(),
        ));

        let event = Event::new(
            EventId::new("event_1".to_string()),
            "Test Event".to_string(),
            EventType::Simple,
            "Description".to_string(),
            conditions,
            effects,
            1.0,
        );

        assert_eq!(event.id.id, "event_1");
        assert_eq!(event.name, "Test Event");
        assert_eq!(event.event_type, EventType::Simple);
        assert_eq!(event.conditions.len(), 1);
        assert_eq!(event.effects.len(), 1);
    }

    #[test]
    fn test_add_condition_to_event() {
        let mut event = Event::new(
            EventId::new("event_1".to_string()),
            "Test".to_string(),
            EventType::Simple,
            "Desc".to_string(),
            Vec::new(),
            Vec::new(),
            1.0,
        );

        event.add_condition(Condition::new("always_true".to_string(), HashMap::new()));
        assert_eq!(event.conditions.len(), 1);
    }

    #[test]
    fn test_add_effect_to_event() {
        let mut event = Event::new(
            EventId::new("event_1".to_string()),
            "Test".to_string(),
            EventType::Simple,
            "Desc".to_string(),
            Vec::new(),
            Vec::new(),
            1.0,
        );

        event.add_effect(Effect::new(
            "set_state".to_string(),
            [("key".to_string(), "value".to_string())]
                .iter()
                .cloned()
                .collect(),
        ));
        assert_eq!(event.effects.len(), 1);
    }
}
