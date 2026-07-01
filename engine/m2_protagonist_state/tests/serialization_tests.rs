use m2_protagonist_state::models::{Inventory, InventoryItem, Medidores, Objective, Objectives, Perfil, Protagonist, ProtagonistId, Relationships, Tendencia};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_protagonist() -> Protagonist {
        let medidores = Medidores::new(50, 30, 20, 40, 60, 80, Tendencia::new(0));
        let mut inventory = Inventory::new();
        inventory.add_item(
            "item_1".to_string(),
            InventoryItem::new("resource".to_string(), 5),
        );

        let mut relationships = Relationships::new();
        relationships.add_faction_relationship("faction_1".to_string(), 50);

        let mut objectives = Objectives::new();
        objectives.add_objective(
            "obj_1".to_string(),
            Objective::new("Complete mission".to_string()),
        );

        let perfil = Perfil::new(
            "Liberales".to_string(),
            "Burguesia".to_string(),
            "Comerciante".to_string(),
            "Constitucionalista".to_string(),
        );

        Protagonist::new(
            ProtagonistId::new("protagonist_1".to_string()),
            "Test Protagonist".to_string(),
            medidores,
            inventory,
            relationships,
            objectives,
            "normal".to_string(),
            perfil,
            vec!["test".to_string()],
        )
    }

    #[test]
    fn test_serialize_to_json() {
        let protagonist = create_sample_protagonist();
        let json = serde_json::to_string(&protagonist).unwrap();
        assert!(json.contains("protagonist_1"));
        assert!(json.contains("Test Protagonist"));
    }

    #[test]
    fn test_deserialize_from_json() {
        let protagonist = create_sample_protagonist();
        let json = serde_json::to_string(&protagonist).unwrap();
        let deserialized: Protagonist = serde_json::from_str(&json).unwrap();
        assert_eq!(protagonist.id.id, deserialized.id.id);
        assert_eq!(protagonist.name, deserialized.name);
    }

    #[test]
    fn test_serialize_to_yaml() {
        let protagonist = create_sample_protagonist();
        let yaml = serde_yaml::to_string(&protagonist).unwrap();
        assert!(yaml.contains("protagonist_1"));
        assert!(yaml.contains("Test Protagonist"));
    }

    #[test]
    fn test_deserialize_from_yaml() {
        let protagonist = create_sample_protagonist();
        let yaml = serde_yaml::to_string(&protagonist).unwrap();
        let deserialized: Protagonist = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(protagonist.id.id, deserialized.id.id);
    }

    #[test]
    fn test_serialize_to_messagepack() {
        let protagonist = create_sample_protagonist();
        let msgpack = rmp_serde::to_vec(&protagonist).unwrap();
        assert!(!msgpack.is_empty());
    }

    #[test]
    fn test_deserialize_from_messagepack() {
        let protagonist = create_sample_protagonist();
        let msgpack = rmp_serde::to_vec(&protagonist).unwrap();
        let deserialized: Protagonist = rmp_serde::from_slice(&msgpack).unwrap();
        assert_eq!(protagonist.id.id, deserialized.id.id);
    }
}