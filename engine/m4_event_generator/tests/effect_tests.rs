use m4_event_generator::{Effect, EventContext, EventError};
use serde_json::Value;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_flag_effect() {
        let effect = Effect {
            action: "set_flag".to_string(),
            target: "key".to_string(),
            value: Some(Value::Bool(true)),
            delta: None,
        };

        let mut context = EventContext::new();
        assert!(effect.apply(&mut context).is_ok());
        assert_eq!(
            context.get_path("key"),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn test_modify_medidor_effect() {
        let effect = Effect {
            action: "modify_medidor".to_string(),
            target: "key".to_string(),
            value: None,
            delta: Some(5),
        };

        let mut context = EventContext::new();
        context.set_path("key", Value::from(10)).unwrap();

        assert!(effect.apply(&mut context).is_ok());
        assert_eq!(
            context.get_path("key"),
            Some(&Value::from(15))
        );
    }

    #[test]
    fn test_modify_medidor_effect_new_key() {
        let effect = Effect {
            action: "modify_medidor".to_string(),
            target: "key".to_string(),
            value: None,
            delta: Some(10),
        };

        let mut context = EventContext::new();

        assert!(effect.apply(&mut context).is_ok());
        assert_eq!(
            context.get_path("key"),
            Some(&Value::from(10))
        );
    }

    #[test]
    fn test_unknown_effect() {
        let effect = Effect {
            action: "unknown".to_string(),
            target: "key".to_string(),
            value: None,
            delta: None,
        };
        let mut context = EventContext::new();
        let result = effect.apply(&mut context);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            EventError::EffectApplicationError("Unknown effect action: unknown".to_string())
        );
    }
}