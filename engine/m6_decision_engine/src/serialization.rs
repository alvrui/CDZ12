use crate::error::DecisionError;
use crate::models::DecisionTree;

pub fn serialize_to_json(tree: &DecisionTree) -> Result<String, DecisionError> {
    serde_json::to_string(tree)
        .map_err(|e| DecisionError::SerializationError(e.to_string()))
}

pub fn deserialize_from_json(data: &str) -> Result<DecisionTree, DecisionError> {
    serde_json::from_str(data)
        .map_err(|e| DecisionError::SerializationError(e.to_string()))
}

pub fn serialize_to_yaml(tree: &DecisionTree) -> Result<String, DecisionError> {
    serde_yaml::to_string(tree)
        .map_err(|e| DecisionError::SerializationError(e.to_string()))
}

pub fn deserialize_from_yaml(data: &str) -> Result<DecisionTree, DecisionError> {
    serde_yaml::from_str(data)
        .map_err(|e| DecisionError::SerializationError(e.to_string()))
}

pub fn serialize_to_msgpack(tree: &DecisionTree) -> Result<Vec<u8>, DecisionError> {
    rmp_serde::to_vec(tree)
        .map_err(|e| DecisionError::SerializationError(e.to_string()))
}

pub fn deserialize_from_msgpack(data: &[u8]) -> Result<DecisionTree, DecisionError> {
    rmp_serde::from_slice(data)
        .map_err(|e| DecisionError::SerializationError(e.to_string()))
}
