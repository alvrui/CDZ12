#[cfg(test)]
mod faction_tests {
    use m1_world_state::entities::faction::{Faction, FactionId, RelationValue};
    
    // Test: Creacion basica
    #[test]
    fn test_create_faction() {
        let faction = Faction::new(FactionId::from("faccion_1"));
        assert_eq!(faction.id().as_str(), "faccion_1");
        assert_eq!(faction.strength(), 50); // Valor por defecto
    }
    
    // Test: Fuerza
    #[test]
    fn test_faction_strength() {
        let mut faction = Faction::new(FactionId::from("f1"));
        faction.set_strength(75);
        assert_eq!(faction.strength(), 75);
        
        faction.set_strength(200); // Se clamp a 100
        assert_eq!(faction.strength(), 100);
    }
    
    // Test: Relaciones
    #[test]
    fn test_faction_relations() {
        let mut faction1 = Faction::new(FactionId::from("f1"));
        let faction2_id = FactionId::from("f2");
        
        faction1.set_relation(faction2_id.clone(), RelationValue::new(60));
        assert_eq!(
            faction1.get_relation(&faction2_id).unwrap().value(),
            60
        );
        assert!(faction1.is_allied(&faction2_id));
        
        faction1.set_relation(faction2_id.clone(), RelationValue::new(-60));
        assert!(faction1.is_enemy(&faction2_id));
    }
    
    // Test: Relacion por defecto
    #[test]
    fn test_default_relation() {
        let faction = Faction::new(FactionId::from("f1"));
        let other_id = FactionId::from("f2");
        assert_eq!(faction.get_relation(&other_id), None);
        assert_eq!(faction.get_relation_or_default(&other_id).value(), 0);
    }
    
    // Test: Serializacion
    #[test]
    fn test_faction_serialization() {
        let mut faction = Faction::new(FactionId::from("f1"));
        faction.set_strength(80);
        faction.set_relation(FactionId::from("f2"), RelationValue::new(60));
        
        let serialized = serde_json::to_string(&faction).unwrap();
        let deserialized: Faction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(faction.id(), deserialized.id());
        assert_eq!(faction.strength(), deserialized.strength());
    }
}
