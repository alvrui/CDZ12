use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ProtagonistError {
    #[error("Attribute {0} out of bounds. Must be between {1} and {2}")]
    AttributeOutOfBounds(String, i32, i32),

    #[error("Inventory item {0} not found")]
    ItemNotFound(String),

    #[error("Objective {0} not found")]
    ObjectiveNotFound(String),

    #[error("Relationship with faction {0} not found")]
    RelationshipNotFound(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}
