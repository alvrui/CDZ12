use crate::error::EventError;
use crate::models::EventPool;

pub fn serialize_to_json(pool: &EventPool) -> Result<String, EventError> {
    serde_json::to_string(pool)
        .map_err(|e| EventError::SerializationError(e.to_string()))
}

pub fn deserialize_from_json(data: &str) -> Result<EventPool, EventError> {
    serde_json::from_str(data)
        .map_err(|e| EventError::SerializationError(e.to_string()))
}

pub fn serialize_to_yaml(pool: &EventPool) -> Result<String, EventError> {
    serde_yaml::to_string(pool)
        .map_err(|e| EventError::SerializationError(e.to_string()))
}

pub fn deserialize_from_yaml(data: &str) -> Result<EventPool, EventError> {
    serde_yaml::from_str(data)
        .map_err(|e| EventError::SerializationError(e.to_string()))
}

pub fn serialize_to_msgpack(pool: &EventPool) -> Result<Vec<u8>, EventError> {
    rmp_serde::to_vec(pool)
        .map_err(|e| EventError::SerializationError(e.to_string()))
}

pub fn deserialize_from_msgpack(data: &[u8]) -> Result<EventPool, EventError> {
    rmp_serde::from_slice(data)
        .map_err(|e| EventError::SerializationError(e.to_string()))
}
