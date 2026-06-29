//! Serialización para el módulo M1

use serde::{Serialize, Deserialize};
use crate::models::WorldState;

/// Formato de serialización
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SerializationFormat {
    /// Formato JSON
    Json,
    /// Formato YAML
    Yaml,
    /// Formato MessagePack (binario)
    MessagePack,
}

/// Error de serialización
#[derive(Debug, thiserror::Error)]
pub enum SerializationError {
    /// Error de JSON
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    /// Error de YAML
    #[error("YAML serialization error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    /// Error de MessagePack (serialización)
    #[error("MessagePack serialization error: {0}")]
    MessagePackEncode(#[from] rmp_serde::encode::Error),
    /// Error de MessagePack (deserialización)
    #[error("MessagePack deserialization error: {0}")]
    MessagePackDecode(#[from] rmp_serde::decode::Error),
    /// UTF-8 inválido
    #[error("Invalid UTF-8")]
    InvalidUtf8,
}

impl WorldState {
    /// Serializa el estado del mundo al formato especificado
    pub fn serialize(&self, format: SerializationFormat) -> Result<Vec<u8>, SerializationError> {
        match format {
            SerializationFormat::Json => {
                Ok(serde_json::to_vec(self)?)
            }
            SerializationFormat::Yaml => {
                Ok(serde_yaml::to_string(self)?.into_bytes())
            }
            SerializationFormat::MessagePack => {
                Ok(rmp_serde::to_vec(self)?)
            }
        }
    }

    /// Deserializa el estado del mundo desde el formato especificado
    pub fn deserialize(data: &[u8], format: SerializationFormat) -> Result<Self, SerializationError> {
        match format {
            SerializationFormat::Json => {
                Ok(serde_json::from_slice(data)?)
            }
            SerializationFormat::Yaml => {
                let yaml = std::str::from_utf8(data)
                    .map_err(|_| SerializationError::InvalidUtf8)?;
                Ok(serde_yaml::from_str(yaml)?)
            }
            SerializationFormat::MessagePack => {
                Ok(rmp_serde::from_slice(data)?)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_state_json_serialization() {
        let world = WorldState::new();
        let data = world.serialize(SerializationFormat::Json).unwrap();
        let deserialized = WorldState::deserialize(&data, SerializationFormat::Json).unwrap();
        assert_eq!(world.time().jornada_absoluta(), deserialized.time().jornada_absoluta());
    }

    #[test]
    fn test_world_state_yaml_serialization() {
        let world = WorldState::new();
        let data = world.serialize(SerializationFormat::Yaml).unwrap();
        let deserialized = WorldState::deserialize(&data, SerializationFormat::Yaml).unwrap();
        assert_eq!(world.time().jornada_absoluta(), deserialized.time().jornada_absoluta());
    }

    #[test]
    fn test_world_state_msgpack_serialization() {
        let world = WorldState::new();
        let data = world.serialize(SerializationFormat::MessagePack).unwrap();
        let deserialized = WorldState::deserialize(&data, SerializationFormat::MessagePack).unwrap();
        assert_eq!(world.time().jornada_absoluta(), deserialized.time().jornada_absoluta());
    }
}
