use crate::models::{Medidor, Medidores};
use crate::MedidorError;

pub fn serialize_to_json(medidores: &Medidores) -> Result<String, MedidorError> {
    serde_json::to_string(medidores)
        .map_err(|e| MedidorError::SerializationError(e.to_string()))
}

pub fn deserialize_from_json(data: &str) -> Result<Medidores, MedidorError> {
    serde_json::from_str(data)
        .map_err(|e| MedidorError::SerializationError(e.to_string()))
}

pub fn serialize_to_yaml(medidores: &Medidores) -> Result<String, MedidorError> {
    serde_yaml::to_string(medidores)
        .map_err(|e| MedidorError::SerializationError(e.to_string()))
}

pub fn deserialize_from_yaml(data: &str) -> Result<Medidores, MedidorError> {
    serde_yaml::from_str(data)
        .map_err(|e| MedidorError::SerializationError(e.to_string()))
}

pub fn serialize_to_msgpack(medidores: &Medidores) -> Result<Vec<u8>, MedidorError> {
    rmp_serde::to_vec(medidores)
        .map_err(|e| MedidorError::SerializationError(e.to_string()))
}

pub fn deserialize_from_msgpack(data: &[u8]) -> Result<Medidores, MedidorError> {
    rmp_serde::from_slice(data)
        .map_err(|e| MedidorError::SerializationError(e.to_string()))
}
