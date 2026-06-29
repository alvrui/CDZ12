use std::path::Path;
use std::fs;
use crate::error::NarrativeError;
use crate::models::{GameContext, NarrativeConfig, NarrativeRequest};
use crate::triggers::TriggerStore;
use crate::models::NarrativeTrigger;

pub struct NarrativeGenerator {
    trigger_store: TriggerStore,
    config_dir: String,
}

impl NarrativeGenerator {
    pub fn new(config_dir: String) -> Self {
        Self {
            trigger_store: TriggerStore::new(),
            config_dir,
        }
    }

    pub fn add_trigger(&mut self, trigger: NarrativeTrigger) {
        self.trigger_store.add_trigger(trigger);
    }

    pub fn check_and_generate(&self, context: &GameContext) -> Result<Vec<NarrativeConfig>, NarrativeError> {
        let triggered = self.trigger_store.check_triggers(context)?;
        let mut configs = Vec::new();

        for trigger in triggered {
            let request = NarrativeRequest::new(trigger.id.clone(), context.clone());
            self.save_request(&request)?;
            if let Some(config) = self.load_config(&trigger.id.id)? {
                configs.push(config);
            }
        }

        Ok(configs)
    }

    fn save_request(&self, request: &NarrativeRequest) -> Result<(), NarrativeError> {
        let path = Path::new(&self.config_dir)
            .join("requests")
            .join(format!("{}.json", request.trigger_id.id));

        std::fs::create_dir_all(path.parent().unwrap()).map_err(|e| {
            NarrativeError::SdkCommunicationError(format!("Failed to create directory: {}", e))
        })?;

        let json = serde_json::to_string_pretty(request)
            .map_err(|e| NarrativeError::SerializationError(e.to_string()))?;
        std::fs::write(&path, json)
            .map_err(|e| NarrativeError::SdkCommunicationError(format!("Failed to write request: {}", e)))?;

        Ok(())
    }

    fn load_config(&self, trigger_id: &str) -> Result<Option<NarrativeConfig>, NarrativeError> {
        let path = Path::new(&self.config_dir)
            .join("responses")
            .join(format!("{}.json", trigger_id));

        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| NarrativeError::ConfigNotFound(format!("Failed to read config: {}", e)))?;

        let config: NarrativeConfig = serde_json::from_str(&content)
            .map_err(|e| NarrativeError::SerializationError(e.to_string()))?;

        Ok(Some(config))
    }

    pub fn load_triggers_from_file(&mut self, path: &str) -> Result<(), NarrativeError> {
        let content = fs::read_to_string(path)
            .map_err(|e| NarrativeError::ConfigNotFound(format!("Failed to read triggers: {}", e)))?;

        let triggers: Vec<NarrativeTrigger> = serde_json::from_str(&content)
            .map_err(|e| NarrativeError::SerializationError(e.to_string()))?;

        for trigger in triggers {
            self.trigger_store.add_trigger(trigger);
        }

        Ok(())
    }

    pub fn get_trigger_store(&self) -> &TriggerStore {
        &self.trigger_store
    }
}
