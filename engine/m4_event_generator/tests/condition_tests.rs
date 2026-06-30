use m4_event_generator::Condition;
use m4_event_generator::EventContext;
use serde_json::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_true_condition() {
        let condition = Condition::AlwaysTrue;
        let context = EventContext::new();
        assert!(condition.evaluate(&context));
    }

    #[test]
    fn test_state_equals_condition_true() {
        let condition = Condition::StateEquals {
            key: "polarization".to_string(),
            value: "50".to_string(),
        };

        let mut context = EventContext::new();
        context.set_path("polarization", Value::String("50".to_string())).unwrap();

        assert!(condition.evaluate(&context));
    }

    #[test]
    fn test_state_equals_condition_false() {
        let condition = Condition::StateEquals {
            key: "polarization".to_string(),
            value: "50".to_string(),
        };

        let mut context = EventContext::new();
        context.set_path("polarization", Value::String("30".to_string())).unwrap();

        assert!(!condition.evaluate(&context));
    }

    #[test]
    fn test_state_greater_than_condition_true() {
        let condition = Condition::StateGreaterThan {
            key: "polarization".to_string(),
            threshold: 50,
        };

        let mut context = EventContext::new();
        context.set_path("polarization", Value::from(60)).unwrap();

        assert!(condition.evaluate(&context));
    }

    #[test]
    fn test_state_greater_than_condition_false() {
        let condition = Condition::StateGreaterThan {
            key: "polarization".to_string(),
            threshold: 50,
        };

        let mut context = EventContext::new();
        context.set_path("polarization", Value::from(40)).unwrap();

        assert!(!condition.evaluate(&context));
    }
}