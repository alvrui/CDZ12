use crate::error::ProtagonistError;
use crate::models::Relationships;

impl Relationships {
    pub fn add_relationship(&mut self, faction_id: String, value: i32) {
        self.factions.insert(faction_id, value);
    }

    pub fn update_relationship(
        &mut self,
        faction_id: &str,
        value: i32,
    ) -> Result<(), ProtagonistError> {
        if self.factions.contains_key(faction_id) {
            self.factions.insert(faction_id.to_string(), value);
            Ok(())
        } else {
            Err(ProtagonistError::RelationshipNotFound(faction_id.to_string()))
        }
    }

    pub fn increase_relationship(
        &mut self,
        faction_id: &str,
        amount: i32,
    ) -> Result<(), ProtagonistError> {
        if let Some(current) = self.factions.get_mut(faction_id) {
            *current += amount;
            Ok(())
        } else {
            Err(ProtagonistError::RelationshipNotFound(faction_id.to_string()))
        }
    }

    pub fn get_relationship(&self, faction_id: &str) -> Result<i32, ProtagonistError> {
        self.factions
            .get(faction_id)
            .copied()
            .ok_or_else(|| ProtagonistError::RelationshipNotFound(faction_id.to_string()))
    }

    pub fn remove_relationship(&mut self, faction_id: &str) -> Result<(), ProtagonistError> {
        if self.factions.contains_key(faction_id) {
            self.factions.remove(faction_id);
            Ok(())
        } else {
            Err(ProtagonistError::RelationshipNotFound(faction_id.to_string()))
        }
    }
}
