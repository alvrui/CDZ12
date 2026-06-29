#[cfg(test)]
mod space_tests {
    use m1_world_state::entities::space::{Space, SpaceId, SpaceType, Accessibility};
    use m1_world_state::entities::FactionId;
    
    // Test: Creacion basica
    #[test]
    fn test_create_space() {
        let space = Space::new(
            SpaceId::from("espacio_1"),
            SpaceType::Public,
            Accessibility::Open
        );
        assert_eq!(space.id().as_str(), "espacio_1");
        assert!(matches!(space.space_type(), SpaceType::Public));
        assert!(matches!(space.accessibility(), Accessibility::Open));
    }
    
    // Test: Tipos de espacio
    #[test]
    fn test_space_types() {
        let public = Space::new(SpaceId::from("s1"), SpaceType::Public, Accessibility::Open);
        let private = Space::new(SpaceId::from("s2"), SpaceType::Private, Accessibility::Restricted);
        let secret = Space::new(SpaceId::from("s3"), SpaceType::Secret, Accessibility::Hidden);
        
        assert!(matches!(public.space_type(), SpaceType::Public));
        assert!(matches!(private.space_type(), SpaceType::Private));
        assert!(matches!(secret.space_type(), SpaceType::Secret));
    }
    
    // Test: Conexiones
    #[test]
    fn test_space_connections() {
        let mut space1 = Space::new(SpaceId::from("s1"), SpaceType::Public, Accessibility::Open);
        let space2_id = SpaceId::from("s2");
        
        space1.add_connection(space2_id.clone());
        assert!(space1.is_connected_to(&space2_id));
        assert_eq!(space1.connected_spaces().len(), 1);
    }
    
    // Test: Requerimientos
    #[test]
    fn test_space_requirements() {
        let mut space = Space::new(SpaceId::from("s1"), SpaceType::Private, Accessibility::Restricted);
        space.add_required_faction(FactionId::from("f1"));
        space.add_required_medidor("influencia".to_string(), 50);
        
        assert!(space.requires_faction(&FactionId::from("f1")));
        assert_eq!(space.required_medidor("influencia"), Some(50));
    }
    
    // Test: Peligrosidad
    #[test]
    fn test_space_danger() {
        let mut space = Space::new(SpaceId::from("s1"), SpaceType::Public, Accessibility::Open);
        space.set_danger_level(75);
        assert_eq!(space.danger_level(), 75);
        
        space.set_danger_level(150); // Se clamp a 100
        assert_eq!(space.danger_level(), 100);
    }
    
    // Test: Serializacion
    #[test]
    fn test_space_serialization() {
        let mut space = Space::new(SpaceId::from("s1"), SpaceType::Public, Accessibility::Open);
        space.set_danger_level(30);
        
        let serialized = serde_json::to_string(&space).unwrap();
        let deserialized: Space = serde_json::from_str(&serialized).unwrap();
        assert_eq!(space.id(), deserialized.id());
        assert_eq!(space.danger_level(), deserialized.danger_level());
    }
}
