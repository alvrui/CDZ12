use crate::error::ProtagonistError;
use crate::models::Attributes;

impl Attributes {
    pub const MIN_VALUE: i32 = 0;
    pub const MAX_VALUE: i32 = 100;

    pub fn increase_strength(&mut self, value: i32) -> Result<(), ProtagonistError> {
        self.strength += value;
        self.validate_strength()
    }

    pub fn decrease_strength(&mut self, value: i32) -> Result<(), ProtagonistError> {
        self.strength -= value;
        self.validate_strength()
    }

    pub fn increase_influence(&mut self, value: i32) -> Result<(), ProtagonistError> {
        self.influence += value;
        self.validate_influence()
    }

    pub fn decrease_influence(&mut self, value: i32) -> Result<(), ProtagonistError> {
        self.influence -= value;
        self.validate_influence()
    }

    pub fn increase_resources(&mut self, value: i32) -> Result<(), ProtagonistError> {
        self.resources += value;
        self.validate_resources()
    }

    pub fn decrease_resources(&mut self, value: i32) -> Result<(), ProtagonistError> {
        self.resources -= value;
        self.validate_resources()
    }

    fn validate_strength(&self) -> Result<(), ProtagonistError> {
        if self.strength < Self::MIN_VALUE || self.strength > Self::MAX_VALUE {
            Err(ProtagonistError::AttributeOutOfBounds(
                "strength".to_string(),
                Self::MIN_VALUE,
                Self::MAX_VALUE,
            ))
        } else {
            Ok(())
        }
    }

    fn validate_influence(&self) -> Result<(), ProtagonistError> {
        if self.influence < Self::MIN_VALUE || self.influence > Self::MAX_VALUE {
            Err(ProtagonistError::AttributeOutOfBounds(
                "influence".to_string(),
                Self::MIN_VALUE,
                Self::MAX_VALUE,
            ))
        } else {
            Ok(())
        }
    }

    fn validate_resources(&self) -> Result<(), ProtagonistError> {
        if self.resources < Self::MIN_VALUE || self.resources > Self::MAX_VALUE {
            Err(ProtagonistError::AttributeOutOfBounds(
                "resources".to_string(),
                Self::MIN_VALUE,
                Self::MAX_VALUE,
            ))
        } else {
            Ok(())
        }
    }
}
