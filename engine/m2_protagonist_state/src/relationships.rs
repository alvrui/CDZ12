use crate::error::ProtagonistError;
use crate::models::Relationships;

impl Relationships {
    pub fn add_faction_relationship(&mut self, faction_id: String, value: i32) {
        self.factions.insert(faction_id, value);
    }

    pub fn add_npc_relationship(&mut self, npc_id: String, value: i32) {
        self.npcs.insert(npc_id, value);
    }

    pub fn update_faction_relationship(
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

    pub fn update_npc_relationship(
        &mut self,
        npc_id: &str,
        value: i32,
    ) -> Result<(), ProtagonistError> {
        if self.npcs.contains_key(npc_id) {
            self.npcs.insert(npc_id.to_string(), value);
            Ok(())
        } else {
            Err(ProtagonistError::RelationshipNotFound(npc_id.to_string()))
        }
    }

    pub fn increase_faction_relationship(
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

    pub fn increase_npc_relationship(
        &mut self,
        npc_id: &str,
        amount: i32,
    ) -> Result<(), ProtagonistError> {
        if let Some(current) = self.npcs.get_mut(npc_id) {
            *current += amount;
            Ok(())
        } else {
            Err(ProtagonistError::RelationshipNotFound(npc_id.to_string()))
        }
    }

    pub fn get_faction_relationship(&self, faction_id: &str) -> Result<i32, ProtagonistError> {
        self.factions
            .get(faction_id)
            .copied()
            .ok_or_else(|| ProtagonistError::RelationshipNotFound(faction_id.to_string()))
    }

    pub fn get_npc_relationship(&self, npc_id: &str) -> Result<i32, ProtagonistError> {
        self.npcs
            .get(npc_id)
            .copied()
            .ok_or_else(|| ProtagonistError::RelationshipNotFound(npc_id.to_string()))
    }

    pub fn remove_faction_relationship(&mut self, faction_id: &str) -> Result<(), ProtagonistError> {
        if self.factions.contains_key(faction_id) {
            self.factions.remove(faction_id);
            Ok(())
        } else {
            Err(ProtagonistError::RelationshipNotFound(faction_id.to_string()))
        }
    }

    pub fn remove_npc_relationship(&mut self, npc_id: &str) -> Result<(), ProtagonistError> {
        if self.npcs.contains_key(npc_id) {
            self.npcs.remove(npc_id);
            Ok(())
        } else {
            Err(ProtagonistError::RelationshipNotFound(npc_id.to_string()))
        }
    }
}