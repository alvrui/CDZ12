use m7_narrative_generator::{ConfigManager, GameContext, NarrativeConfig, NarrativeGenerator, NarrativeRequest, NarrativeTrigger, TriggerCondition, TriggerId, TriggerStore, VariableType, ConditionOperator};
use std::collections::HashMap;
use std::fs;
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_trigger() -> NarrativeTrigger {
        let mut conditions = Vec::new();
        conditions.push(TriggerCondition::new(
            "polarization".to_string(),
            VariableType::Medidor,
            ConditionOperator::GreaterThan,
            "80".to_string(),
        ));
        NarrativeTrigger::new(
            TriggerId::new("test_trigger".to_string()),
            "Test Trigger".to_string(),
            "Test".to_string(),
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
    fn test_create_generator() {
        let generator = NarrativeGenerator::new("config".to_string());
        assert!(generator.get_trigger_store().triggers.is_empty());
    }

    #[test]
    fn test_add_trigger_to_generator() {
        let mut generator = NarrativeGenerator::new("config".to_string());
        generator.add_trigger(create_test_trigger());
        assert_eq!(generator.get_trigger_store().triggers.len(), 1);
    }

    #[test]
    fn test_check_and_generate_no_trigger() {
        let mut generator = NarrativeGenerator::new("config".to_string());
        generator.add_trigger(create_test_trigger());

        let mut context = GameContext::new();
        context.medidores.insert("polarization".to_string(), "70".to_string());

        let result = generator.check_and_generate(&context);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_load_triggers_from_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("triggers.json");

        let trigger = create_test_trigger();
        let triggers = vec![trigger];
        let json = serde_json::to_string(&triggers).unwrap();
        fs::write(&file_path, json).unwrap();

        let mut generator = NarrativeGenerator::new(dir.path().to_str().unwrap().to_string());
        assert!(generator.load_triggers_from_file(file_path.to_str().unwrap()).is_ok());
        assert_eq!(generator.get_trigger_store().triggers.len(), 1);
    }

    #[test]
    fn test_config_manager_load() {
        let dir = tempdir().unwrap();
        let responses_dir = dir.path().join("responses");
        fs::create_dir_all(&responses_dir).unwrap();

        let config = NarrativeConfig::new(
            "req_1".to_string(),
            TriggerId::new("test_trigger".to_string()),
            vec![],
            "Test narrative".to_string(),
            vec![],
            vec![],
        );
        let json = serde_json::to_string(&config).unwrap();
        fs::write(responses_dir.join("test_trigger.json"), json).unwrap();

        let manager = ConfigManager::new(dir.path().to_str().unwrap().to_string());
        let result = manager.load_config("test_trigger");
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_config_manager_list() {
        let dir = tempdir().unwrap();
        let responses_dir = dir.path().join("responses");
        fs::create_dir_all(&responses_dir).unwrap();

        // Crear archivos de prueba
        fs::write(responses_dir.join("trigger1.json"), "{}").unwrap();
        fs::write(responses_dir.join("trigger2.json"), "{}").unwrap();
        fs::write(responses_dir.join("not_json.txt"), "text").unwrap();

        let manager = ConfigManager::new(dir.path().to_str().unwrap().to_string());
        let configs = manager.list_configs().unwrap();
        assert_eq!(configs.len(), 2);
        assert!(configs.contains(&"trigger1.json".to_string()));
        assert!(configs.contains(&"trigger2.json".to_string()));
    }
}
