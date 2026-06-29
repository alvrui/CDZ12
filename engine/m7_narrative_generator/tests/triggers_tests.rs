use m7_narrative_generator::{ConditionOperator, GameContext, NarrativeTrigger, TriggerCondition, TriggerId, TriggerStore, VariableType};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_trigger() -> NarrativeTrigger {
        let mut conditions = Vec::new();

        // Condición: medidor "polarization" > 80
        conditions.push(TriggerCondition::new(
            "polarization".to_string(),
            VariableType::Medidor,
            ConditionOperator::GreaterThan,
            "80".to_string(),
        ));

        NarrativeTrigger::new(
            TriggerId::new("trigger_1".to_string()),
            "Alta polarización".to_string(),
            "Se activa cuando la polarización supera 80".to_string(),
            conditions,
            1,
        )
    }

    fn create_test_context() -> GameContext {
        let mut context = GameContext::new();
        context.medidores.insert("polarization".to_string(), "85".to_string());
        context
    }

    #[test]
    fn test_create_trigger_store() {
        let store = TriggerStore::new();
        assert!(store.triggers.is_empty());
    }

    #[test]
    fn test_add_trigger() {
        let mut store = TriggerStore::new();
        let trigger = create_test_trigger();
        store.add_trigger(trigger);
        assert_eq!(store.triggers.len(), 1);
    }

    #[test]
    fn test_evaluate_trigger_true() {
        let mut store = TriggerStore::new();
        store.add_trigger(create_test_trigger());
        let context = create_test_context();

        let result = store.evaluate_trigger("trigger_1", &context);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_evaluate_trigger_false() {
        let mut store = TriggerStore::new();
        store.add_trigger(create_test_trigger());

        let mut context = GameContext::new();
        context.medidores.insert("polarization".to_string(), "70".to_string());

        let result = store.evaluate_trigger("trigger_1", &context);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_check_triggers() {
        let mut store = TriggerStore::new();
        store.add_trigger(create_test_trigger());

        let context = create_test_context();
        let triggered = store.check_triggers(&context).unwrap();

        assert_eq!(triggered.len(), 1);
        assert_eq!(triggered[0].id.id, "trigger_1");
    }

    #[test]
    fn test_evaluate_condition_equals() {
        let mut store = TriggerStore::new();
        let mut conditions = Vec::new();
        conditions.push(TriggerCondition::new(
            "event_active".to_string(),
            VariableType::Event,
            ConditionOperator::Equals,
            "true".to_string(),
        ));

        let trigger = NarrativeTrigger::new(
            TriggerId::new("test".to_string()),
            "Test".to_string(),
            "Test".to_string(),
            conditions,
            1,
        );
        store.add_trigger(trigger);

        let mut context = GameContext::new();
        context.active_events.push("event_active".to_string());

        assert!(store.evaluate_trigger("test", &context).unwrap());
    }

    #[test]
    fn test_evaluate_condition_not_equals() {
        let mut store = TriggerStore::new();
        let mut conditions = Vec::new();
        conditions.push(TriggerCondition::new(
            "decision_made".to_string(),
            VariableType::Decision,
            ConditionOperator::NotEquals,
            "true".to_string(),
        ));

        let trigger = NarrativeTrigger::new(
            TriggerId::new("test".to_string()),
            "Test".to_string(),
            "Test".to_string(),
            conditions,
            1,
        );
        store.add_trigger(trigger);

        let context = GameContext::new(); // decision_made no está en el historial

        assert!(store.evaluate_trigger("test", &context).unwrap());
    }
}
