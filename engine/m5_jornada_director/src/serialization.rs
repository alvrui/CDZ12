use crate::error::TiempoError;
use crate::time::Tiempo;

pub fn serialize_to_json(tiempo: &Tiempo) -> Result<String, TiempoError> {
    serde_json::to_string(tiempo)
        .map_err(|e| TiempoError::SerializationError(e.to_string()))
}

pub fn deserialize_from_json(data: &str) -> Result<Tiempo, TiempoError> {
    serde_json::from_str(data)
        .map_err(|e| TiempoError::SerializationError(e.to_string()))
}

pub fn serialize_to_yaml(tiempo: &Tiempo) -> Result<String, TiempoError> {
    serde_yaml::to_string(tiempo)
        .map_err(|e| TiempoError::SerializationError(e.to_string()))
}

pub fn deserialize_from_yaml(data: &str) -> Result<Tiempo, TiempoError> {
    serde_yaml::from_str(data)
        .map_err(|e| TiempoError::SerializationError(e.to_string()))
}

pub fn serialize_to_msgpack(tiempo: &Tiempo) -> Result<Vec<u8>, TiempoError> {
    rmp_serde::to_vec(tiempo)
        .map_err(|e| TiempoError::SerializationError(e.to_string()))
}

pub fn deserialize_from_msgpack(data: &[u8]) -> Result<Tiempo, TiempoError> {
    rmp_serde::from_slice(data)
        .map_err(|e| TiempoError::SerializationError(e.to_string()))
}
