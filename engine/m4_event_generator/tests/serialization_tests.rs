use m4_event_generator::{Condition, Effect, Event, EventId, EventPool, EventType};
use serde_json::Value;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_pool() -> EventPool {
        let mut pool = EventPool::new();

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

        pool.add_event(event);
        pool
    }

    #[test]
    fn test_serialize_to_json() {
        let pool = create_test_pool();
        let json = serde_json::to_string(&pool).unwrap();
        assert!(json.contains("event_1"));
    }

    #[test]
    fn test_deserialize_from_json() {
        let pool = create_test_pool();
        let json = serde_json::to_string(&pool).unwrap();
        let deserialized: EventPool = serde_json::from_str(&json).unwrap();
        assert_eq!(pool.events.len(), deserialized.events.len());
    }

    #[test]
    fn test_serialize_to_yaml() {
        let pool = create_test_pool();
        let yaml = serde_yaml::to_string(&pool).unwrap();
        assert!(yaml.contains("event_1"));
    }

    #[test]
    fn test_serialize_to_messagepack() {
        let pool = create_test_pool();
        let msgpack = rmp_serde::to_vec(&pool).unwrap();
        assert!(!msgpack.is_empty());
    }
}