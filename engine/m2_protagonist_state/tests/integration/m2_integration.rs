use m2_protagonist_state::{
    Attributes, Inventory, InventoryItem, Objective, Objectives, Protagonist, ProtagonistId,
    ProtagonistManager, Relationships,
};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_protagonist() -> Protagonist {
        let mut attributes = Attributes::new(50, 30, 20);
        let mut inventory = Inventory::new();
        inventory.add_item(
            "item_1".to_string(),
            InventoryItem::new("resource".to_string(), 5),
        );

        let mut relationships = Relationships::new();
        relationships.add_relationship("faction_1".to_string(), 50);

        let mut objectives = Objectives::new();
        objectives.add_objective(
            "obj_1".to_string(),
            Objective::new("Complete mission".to_string()),
        );

        Protagonist::new(
            ProtagonistId::new("protagonist_1".to_string()),
            "Test Protagonist".to_string(),
            attributes,
            inventory,
            relationships,
            objectives,
        )
    }

    #[test]
    fn test_protagonist_creation() {
        let protagonist = create_sample_protagonist();
        assert_eq!(protagonist.id.id, "protagonist_1");
        assert_eq!(protagonist.name, "Test Protagonist");
        assert_eq!(protagonist.attributes.strength, 50);
        assert_eq!(protagonist.inventory.items.len(), 1);
        assert_eq!(protagonist.relationships.factions.len(), 1);
        assert_eq!(protagonist.objectives.list.len(), 1);
    }

    #[test]
    fn test_protagonist_manager_creation() {
        let manager = ProtagonistManager::new();
        assert!(manager.protagonists.is_empty());
        assert!(manager.current_protagonist_id.is_none());
    }

    #[test]
    fn test_protagonist_manager_add_and_get() {
        let mut manager = ProtagonistManager::new();
        let protagonist = create_sample_protagonist();
        manager.add_protagonist(protagonist.clone());

        assert!(manager.get_protagonist("protagonist_1").is_some());
        assert_eq!(
            manager.get_protagonist("protagonist_1").unwrap().id.id,
            "protagonist_1"
        );
    }

    #[test]
    fn test_protagonist_manager_current() {
        let mut manager = ProtagonistManager::new();
        let protagonist = create_sample_protagonist();
        manager.add_protagonist(protagonist);

        assert!(manager.set_current_protagonist("protagonist_1").is_ok());
        assert!(manager.get_current_protagonist().is_ok());
        assert_eq!(
            manager.get_current_protagonist().unwrap().id.id,
            "protagonist_1"
        );
    }

    #[test]
    fn test_protagonist_manager_remove() {
        let mut manager = ProtagonistManager::new();
        let protagonist = create_sample_protagonist();
        manager.add_protagonist(protagonist);

        assert!(manager.remove_protagonist("protagonist_1").is_some());
        assert!(manager.get_protagonist("protagonist_1").is_none());
    }

    #[test]
    fn test_protagonist_attributes_operations() {
        let mut protagonist = create_sample_protagonist();
        assert!(protagonist.attributes.increase_strength(10).is_ok());
        assert_eq!(protagonist.attributes.strength, 60);

        assert!(protagonist.attributes.decrease_influence(5).is_ok());
        assert_eq!(protagonist.attributes.influence, 25);
    }

    #[test]
    fn test_protagonist_inventory_operations() {
        let mut protagonist = create_sample_protagonist();
        assert!(protagonist
            .inventory
            .increase_quantity("item_1", 3)
            .is_ok());
        assert_eq!(protagonist.inventory.items["item_1"].quantity, 8);

        assert!(protagonist.inventory.decrease_quantity("item_1", 2).is_ok());
        assert_eq!(protagonist.inventory.items["item_1"].quantity, 6);
    }

    #[test]
    fn test_protagonist_relationships_operations() {
        let mut protagonist = create_sample_protagonist();
        assert!(protagonist
            .relationships
            .increase_relationship("faction_1", 10)
            .is_ok());
        assert_eq!(protagonist.relationships.factions["faction_1"], 60);

        protagonist
            .relationships
            .add_relationship("faction_2".to_string(), 30);
        assert_eq!(protagonist.relationships.factions.len(), 2);
    }

    #[test]
    fn test_protagonist_objectives_operations() {
        let mut protagonist = create_sample_protagonist();
        assert!(protagonist
            .objectives
            .increase_progress("obj_1", 50)
            .is_ok());
        assert_eq!(protagonist.objectives.list["obj_1"].progress, 50);

        assert!(protagonist.objectives.mark_completed("obj_1").is_ok());
        assert!(protagonist.objectives.list["obj_1"].completed);
    }
}
