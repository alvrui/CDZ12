use crate::error::NarrativeError;
use crate::models::{NarrativeConfig, NarrativeRequest};
use crate::triggers::TriggerStore;

pub fn serialize_trigger_store(store: &TriggerStore) -> Result<String, NarrativeError> {
    serde_json::to_string(store)
        .map_err(|e| NarrativeError::SerializationError(e.to_string()))
}

pub fn deserialize_trigger_store(data: &str) -> Result<TriggerStore, NarrativeError> {
    serde_json::from_str(data)
        .map_err(|e| NarrativeError::SerializationError(e.to_string()))
}

pub fn serialize_request(request: &NarrativeRequest) -> Result<String, NarrativeError> {
    serde_json::to_string(request)
        .map_err(|e| NarrativeError::SerializationError(e.to_string()))
}

pub fn deserialize_request(data: &str) -> Result<NarrativeRequest, NarrativeError> {
    serde_json::from_str(data)
        .map_err(|e| NarrativeError::SerializationError(e.to_string()))
}

pub fn serialize_config(config: &NarrativeConfig) -> Result<String, NarrativeError> {
    serde_json::to_string(config)
        .map_err(|e| NarrativeError::SerializationError(e.to_string()))
}

pub fn deserialize_config(data: &str) -> Result<NarrativeConfig, NarrativeError> {
    serde_json::from_str(data)
        .map_err(|e| NarrativeError::SerializationError(e.to_string()))
}
