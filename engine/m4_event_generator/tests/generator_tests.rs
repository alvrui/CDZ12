use m4_event_generator::{Condition, Effect, Event, EventContext, EventGenerator, EventId, EventPool, EventType};
use serde_json::Value;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_pool() -> EventPool {
        let mut pool = EventPool::new();

        // Evento siempre disponible
        let event1 = Event::new(
            EventId::new("event_1".to_string()),
            "Always Available".to_string(),
            EventType::Simple,
            "Always triggers".to_string(),
            vec![Condition::AlwaysTrue],
            vec![],
            1.0,
        );

        // Evento condicional
        let event2 = Event::new(
            EventId::new("event_2".to_string()),
            "Conditional".to_string(),
            EventType::Conditional,
            "Requires state".to_string(),
            vec![Condition::StateEquals {
                key: "key".to_string(),
                value: "required_value".to_string(),
            }],
            vec![],
            2.0,
        );

        pool.add_event(event1);
        pool.add_event(event2);
        pool
    }

    #[test]
    fn test_generate_event_always_available() {
        let pool = create_test_pool();
        let generator = EventGenerator::new(pool);
        let context = EventContext::new();

        let result = generator.generate_event(&context);
        assert!(result.is_ok());
        let event_option = result.unwrap();
        assert!(event_option.is_some());
        assert_eq!(event_option.unwrap().id.id, "event_1");
    }

    #[test]
    fn test_generate_event_conditional_not_met() {
        let pool = create_test_pool();
        let generator = EventGenerator::new(pool);
        let context = EventContext::new();

        let result = generator.generate_event(&context);
        assert!(result.is_ok());
        let event_option = result.unwrap();
        assert!(event_option.is_some());
        assert_eq!(event_option.unwrap().id.id, "event_1");
    }

    #[test]
    fn test_generate_event_conditional_met() {
        let pool = create_test_pool();
        let generator = EventGenerator::new(pool);

        let mut context = EventContext::new();
        context.set_path("key", Value::String("required_value".to_string())).unwrap();

        let result = generator.generate_event(&context);
        assert!(result.is_ok());
        let event_option = result.unwrap();
        assert!(event_option.is_some());
        let event = event_option.unwrap();
        assert!(event.id.id == "event_1" || event.id.id == "event_2");
    }

    #[test]
    fn test_generate_all_events() {
        let pool = create_test_pool();
        let generator = EventGenerator::new(pool);

        let mut context = EventContext::new();
        context.set_path("key", Value::String("required_value".to_string())).unwrap();

        let result = generator.generate_all_events(&context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 2);
    }
}