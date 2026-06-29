use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DecisionError {
    #[error("Node {0} not found")]
    NodeNotFound(String),

    #[error("Option {0} not found")]
    OptionNotFound(String),

    #[error("No options available for node {0}")]
    NoOptionsAvailable(String),

    #[error("Invalid option for current node")]
    InvalidOption,

    #[error("Serialization error: {0}")]
    SerializationError(String),
}
