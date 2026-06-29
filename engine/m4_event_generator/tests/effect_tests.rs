use m4_event_generator::{Effect, EventContext, EventError};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_state_effect() {
        let mut parameters = HashMap::new();
        parameters.insert("key".to_string(), "value".to_string());

        let effect = Effect::new("set_state".to_string(), parameters);

        let mut context = EventContext::new();
        assert!(effect.apply(&mut context).is_ok());
        assert_eq!(context.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_increment_state_effect() {
        let mut parameters = HashMap::new();
        parameters.insert("key".to_string(), "10".to_string());

        let effect = Effect::new("increment_state".to_string(), parameters);

        let mut context = EventContext::new();
        context.set("key".to_string(), "5".to_string());

        assert!(effect.apply(&mut context).is_ok());
        assert_eq!(context.get("key"), Some(&"15".to_string()));
    }

    #[test]
    fn test_increment_state_effect_new_key() {
        let mut parameters = HashMap::new();
        parameters.insert("key".to_string(), "10".to_string());

        let effect = Effect::new("increment_state".to_string(), parameters);

        let mut context = EventContext::new();

        assert!(effect.apply(&mut context).is_ok());
        assert_eq!(context.get("key"), Some(&"10".to_string()));
    }

    #[test]
    fn test_unknown_effect() {
        let effect = Effect::new("unknown".to_string(), HashMap::new());
        let mut context = EventContext::new();
        let result = effect.apply(&mut context);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            EventError::EffectApplicationError("Unknown effect type: unknown".to_string())
        );
    }
}
