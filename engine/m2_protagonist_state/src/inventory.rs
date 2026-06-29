use crate::error::ProtagonistError;
use crate::models::{Inventory, InventoryItem};

impl Inventory {
    pub fn add_item(&mut self, item_id: String, item: InventoryItem) {
        self.items.insert(item_id, item);
    }

    pub fn remove_item(&mut self, item_id: &str) -> Result<InventoryItem, ProtagonistError> {
        self.items
            .remove(item_id)
            .ok_or_else(|| ProtagonistError::ItemNotFound(item_id.to_string()))
    }

    pub fn get_item(&self, item_id: &str) -> Result<&InventoryItem, ProtagonistError> {
        self.items
            .get(item_id)
            .ok_or_else(|| ProtagonistError::ItemNotFound(item_id.to_string()))
    }

    pub fn update_quantity(&mut self, item_id: &str, quantity: u32) -> Result<(), ProtagonistError> {
        if let Some(item) = self.items.get_mut(item_id) {
            item.quantity = quantity;
            Ok(())
        } else {
            Err(ProtagonistError::ItemNotFound(item_id.to_string()))
        }
    }

    pub fn increase_quantity(&mut self, item_id: &str, amount: u32) -> Result<(), ProtagonistError> {
        if let Some(item) = self.items.get_mut(item_id) {
            item.quantity += amount;
            Ok(())
        } else {
            Err(ProtagonistError::ItemNotFound(item_id.to_string()))
        }
    }

    pub fn decrease_quantity(&mut self, item_id: &str, amount: u32) -> Result<(), ProtagonistError> {
        if let Some(item) = self.items.get_mut(item_id) {
            if item.quantity >= amount {
                item.quantity -= amount;
                Ok(())
            } else {
                Err(ProtagonistError::ValidationError(
                    "Cannot decrease quantity below zero".to_string(),
                ))
            }
        } else {
            Err(ProtagonistError::ItemNotFound(item_id.to_string()))
        }
    }
}
