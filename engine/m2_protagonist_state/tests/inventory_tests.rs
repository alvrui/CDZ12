use m2_protagonist_state::{Inventory, InventoryItem, ProtagonistError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_inventory() {
        let inventory = Inventory::new();
        assert!(inventory.items.is_empty());
    }

    #[test]
    fn test_add_item() {
        let mut inventory = Inventory::new();
        let item = InventoryItem::new("resource".to_string(), 5);
        inventory.add_item("item_1".to_string(), item);
        assert_eq!(inventory.items.len(), 1);
        assert_eq!(inventory.items["item_1"].item_type, "resource");
        assert_eq!(inventory.items["item_1"].quantity, 5);
    }

    #[test]
    fn test_remove_item() {
        let mut inventory = Inventory::new();
        let item = InventoryItem::new("resource".to_string(), 5);
        inventory.add_item("item_1".to_string(), item);
        assert!(inventory.remove_item("item_1").is_ok());
        assert_eq!(inventory.items.len(), 0);
    }

    #[test]
    fn test_remove_nonexistent_item() {
        let mut inventory = Inventory::new();
        assert!(inventory.remove_item("item_1").is_err());
        assert_eq!(
            inventory.remove_item("item_1").unwrap_err(),
            ProtagonistError::ItemNotFound("item_1".to_string())
        );
    }

    #[test]
    fn test_update_quantity() {
        let mut inventory = Inventory::new();
        let item = InventoryItem::new("resource".to_string(), 5);
        inventory.add_item("item_1".to_string(), item);
        assert!(inventory.update_quantity("item_1", 10).is_ok());
        assert_eq!(inventory.items["item_1"].quantity, 10);
    }

    #[test]
    fn test_increase_quantity() {
        let mut inventory = Inventory::new();
        let item = InventoryItem::new("resource".to_string(), 5);
        inventory.add_item("item_1".to_string(), item);
        assert!(inventory.increase_quantity("item_1", 3).is_ok());
        assert_eq!(inventory.items["item_1"].quantity, 8);
    }

    #[test]
    fn test_decrease_quantity() {
        let mut inventory = Inventory::new();
        let item = InventoryItem::new("resource".to_string(), 5);
        inventory.add_item("item_1".to_string(), item);
        assert!(inventory.decrease_quantity("item_1", 2).is_ok());
        assert_eq!(inventory.items["item_1"].quantity, 3);
    }

    #[test]
    fn test_decrease_quantity_below_zero() {
        let mut inventory = Inventory::new();
        let item = InventoryItem::new("resource".to_string(), 2);
        inventory.add_item("item_1".to_string(), item);
        assert!(inventory.decrease_quantity("item_1", 5).is_err());
    }
}
