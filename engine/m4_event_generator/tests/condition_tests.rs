use m4_event_generator::{Condition, EventContext};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_always_true_condition() {
        let condition = Condition::new("always_true".to_string(), HashMap::new());
        let context = EventContext::new();
        assert!(condition.evaluate(&context));
    }

    #[test]
    fn test_state_equals_condition_true() {
        let mut parameters = HashMap::new();
        parameters.insert("polarization".to_string(), "50".to_string());

        let condition = Condition::new("state_equals".to_string(), parameters);

        let mut context = EventContext::new();
        context.set("polarization".to_string(), "50".to_string());

        assert!(condition.evaluate(&context));
    }

    #[test]
    fn test_state_equals_condition_false() {
        let mut parameters = HashMap::new();
        parameters.insert("polarization".to_string(), "50".to_string());

        let condition = Condition::new("state_equals".to_string(), parameters);

        let mut context = EventContext::new();
        context.set("polarization".to_string(), "30".to_string());

        assert!(!condition.evaluate(&context));
    }

    #[test]
    fn test_state_greater_than_condition_true() {
        let mut parameters = HashMap::new();
        parameters.insert("polarization".to_string(), "50".to_string());

        let condition = Condition::new("state_greater_than".to_string(), parameters);

        let mut context = EventContext::new();
        context.set("polarization".to_string(), "60".to_string());

        assert!(condition.evaluate(&context));
    }

    #[test]
    fn test_state_greater_than_condition_false() {
        let mut parameters = HashMap::new();
        parameters.insert("polarization".to_string(), "50".to_string());

        let condition = Condition::new("state_greater_than".to_string(), parameters);

        let mut context = EventContext::new();
        context.set("polarization".to_string(), "40".to_string());

        assert!(!condition.evaluate(&context));
    }
}
