use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum TiempoError {
    #[error("Tramo {0} no encontrado")]
    TramoNoEncontrado(String),

    #[error("Acto {0} no encontrado")]
    ActoNoEncontrado(String),

    #[error("Jornada {0} no encontrado")]
    JornadaNoEncontrada(String),

    #[error("No hay más tramos en el acto actual")]
    NoMasTramosEnActo,

    #[error("No hay más actos en la jornada actual")]
    NoMasActosEnJornada,

    #[error("No hay más jornadas disponibles")]
    NoMasJornadas,

    #[error("Error de serialización: {0}")]
    SerializationError(String),
}
impl From<m4_event_generator::error::EventError> for TiempoError {
    fn from(err: m4_event_generator::error::EventError) -> Self {
        TiempoError::SerializationError(err.to_string())
    }
}

impl From<serde_json::Error> for TiempoError {
    fn from(err: serde_json::Error) -> Self {
        TiempoError::SerializationError(err.to_string())
    }
}
