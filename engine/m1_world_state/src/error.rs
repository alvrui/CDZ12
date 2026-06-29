//! Tipos de error para el módulo M1

use thiserror::Error;
use crate::entities::{FactionId, SpaceId, CrisisId};

/// Error principal del módulo M1
#[derive(Debug, Error)]
pub enum M1Error {
    /// Facción no encontrada
    #[error("Faction not found: {0}")]
    FactionNotFound(FactionId),

    /// Espacio no encontrado
    #[error("Space not found: {0}")]
    SpaceNotFound(SpaceId),

    /// Crisis no encontrada
    #[error("Crisis not found: {0}")]
    CrisisNotFound(CrisisId),

    /// Error de tiempo
    #[error("Time error: {0}")]
    TimeError(String),

    /// Error de polarización
    #[error("Polarization error: {0}")]
    PolarizationError(String),

    /// Error de visibilidad
    #[error("Visibility error: {0}")]
    VisibilityError(String),

    /// Error de serialización
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Error de deserialización
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
}

impl M1Error {
    /// Crea un error de facción no encontrada
    pub fn faction_not_found(id: FactionId) -> Self {
        Self::FactionNotFound(id)
    }

    /// Crea un error de espacio no encontrado
    pub fn space_not_found(id: SpaceId) -> Self {
        Self::SpaceNotFound(id)
    }

    /// Crea un error de crisis no encontrada
    pub fn crisis_not_found(id: CrisisId) -> Self {
        Self::CrisisNotFound(id)
    }

    /// Crea un error de tiempo
    pub fn time_error(message: impl Into<String>) -> Self {
        Self::TimeError(message.into())
    }

    /// Crea un error de polarización
    pub fn polarization_error(message: impl Into<String>) -> Self {
        Self::PolarizationError(message.into())
    }

    /// Crea un error de visibilidad
    pub fn visibility_error(message: impl Into<String>) -> Self {
        Self::VisibilityError(message.into())
    }
}

/// Tipo de resultado para operaciones de M1
pub type M1Result<T> = Result<T, M1Error>;
