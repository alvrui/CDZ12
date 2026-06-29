use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum NarrativeError {
    #[error("Trigger {0} not found")]
    TriggerNotFound(String),

    #[error("Config file {0} not found")]
    ConfigNotFound(String),

    #[error("Invalid condition: {0}")]
    InvalidCondition(String),

    #[error("Failed to evaluate condition: {0}")]
    ConditionEvaluationError(String),

    #[error("SDK communication error: {0}")]
    SdkCommunicationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("No triggers defined")]
    NoTriggersDefined,
}
