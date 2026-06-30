use m4_event_generator::{Condition, Effect, Event, EventId, EventType};
use serde_json::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_event() {
        let conditions = vec![Condition::AlwaysTrue];

        let effects = vec![Effect {
            action: "set_flag".to_string(),
            target: "key".to_string(),
            value: Some(Value::Bool(true)),
            delta: None,
        }];

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

        event.add_condition(Condition::AlwaysTrue);
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

        event.add_effect(Effect {
            action: "set_flag".to_string(),
            target: "key".to_string(),
            value: Some(Value::Bool(true)),
            delta: None,
        });
        assert_eq!(event.effects.len(), 1);
    }
}