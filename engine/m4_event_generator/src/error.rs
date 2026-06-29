use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EventError {
    #[error("Event {0} not found")]
    EventNotFound(String),

    #[error("Condition evaluation failed: {0}")]
    ConditionEvaluationError(String),

    #[error("Effect application failed: {0}")]
    EffectApplicationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("File error: {0}")]
    FileError(String),

    #[error("No events available in pool")]
    NoEventsAvailable,
}
