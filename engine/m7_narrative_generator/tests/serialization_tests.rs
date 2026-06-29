use m7_narrative_generator::{GameContext, NarrativeConfig, NarrativeRequest, NarrativeTrigger, TriggerCondition, TriggerId, TriggerStore, VariableType, ConditionOperator, serialize_trigger_store, deserialize_trigger_store, serialize_request, deserialize_request, serialize_config, deserialize_config};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_trigger_store() {
        let mut store = TriggerStore::new();
        let mut conditions = Vec::new();
        conditions.push(TriggerCondition::new(
            "var1".to_string(),
            VariableType::Medidor,
            ConditionOperator::GreaterThan,
            "50".to_string(),
        ));

        let trigger = NarrativeTrigger::new(
            TriggerId::new("trigger_1".to_string()),
            "Test".to_string(),
            "Test".to_string(),
            conditions,
            1,
        );
        store.add_trigger(trigger);

        let json = serialize_trigger_store(&store).unwrap();
        assert!(json.contains("trigger_1"));
    }

    #[test]
    fn test_deserialize_trigger_store() {
        let json = r#"{
            "triggers": {
                "trigger_1": {
                    "id": {"id": "trigger_1"},
                    "name": "Test",
                    "description": "Test",
                    "conditions": [{"variable_name": "var1", "variable_type": "Medidor", "operator": "GreaterThan", "value": "50"}],
                    "priority": 1
                }
            }
        }"#;

        let store: TriggerStore = deserialize_trigger_store(json).unwrap();
        assert_eq!(store.triggers.len(), 1);
    }

    #[test]
    fn test_serialize_request() {
        let context = GameContext::new();
        let request = NarrativeRequest::new(TriggerId::new("test".to_string()), context);
        let json = serialize_request(&request).unwrap();
        assert!(json.contains("test"));
    }

    #[test]
    fn test_serialize_config() {
        let config = NarrativeConfig::new(
            "req_1".to_string(),
            TriggerId::new("trigger_1".to_string()),
            vec![],
            "Test narrative".to_string(),
            vec![],
            vec![],
        );
        let json = serialize_config(&config).unwrap();
        assert!(json.contains("req_1"));
    }
}
