#[cfg(test)]
mod crisis_tests {
    use m1_world_state::entities::crisis::{Crisis, CrisisId, CrisisSeverity, CrisisPhase};
    
    // Test: Creacion basica
    #[test]
    fn test_create_crisis() {
        let crisis = Crisis::new(
            CrisisId::from("crisis_1"),
            CrisisSeverity::High,
            "Descripcion generica".to_string()
        );
        assert_eq!(crisis.id().as_str(), "crisis_1");
        assert!(matches!(crisis.severity(), CrisisSeverity::High));
        assert!(matches!(crisis.current_phase(), CrisisPhase::Pending));
    }
    
    // Test: Severidad
    #[test]
    fn test_crisis_severity() {
        let low = Crisis::new(CrisisId::from("c1"), CrisisSeverity::Low, "".to_string());
        let high = Crisis::new(CrisisId::from("c2"), CrisisSeverity::High, "".to_string());
        
        assert!(matches!(low.severity(), CrisisSeverity::Low));
        assert!(matches!(high.severity(), CrisisSeverity::High));
    }
    
    // Test: Fases
    #[test]
    fn test_crisis_phases() {
        let mut crisis = Crisis::new(CrisisId::from("c1"), CrisisSeverity::Medium, "".to_string());
        
        assert!(matches!(crisis.current_phase(), CrisisPhase::Pending));
        crisis.advance_phase();
        assert!(matches!(crisis.current_phase(), CrisisPhase::Active));
        crisis.advance_phase();
        assert!(matches!(crisis.current_phase(), CrisisPhase::Resolving));
        crisis.advance_phase();
        assert!(matches!(crisis.current_phase(), CrisisPhase::Resolved));
    }
    
    // Test: Duracion
    #[test]
    fn test_crisis_duration() {
        let mut crisis = Crisis::new(CrisisId::from("c1"), CrisisSeverity::High, "".to_string());
        crisis.set_duration(5);
        
        assert_eq!(crisis.duration(), 5);
        assert_eq!(crisis.remaining_jornadas(), 5);
        assert!(!crisis.is_permanent());
        
        crisis.decrease_remaining();
        assert_eq!(crisis.remaining_jornadas(), 4);
    }
    
    // Test: Crisis permanente
    #[test]
    fn test_permanent_crisis() {
        let mut crisis = Crisis::new(CrisisId::from("c1"), CrisisSeverity::High, "".to_string());
        crisis.set_permanent(true);
        
        assert!(crisis.is_permanent());
        assert_eq!(crisis.remaining_jornadas(), 0);
        
        crisis.decrease_remaining();
        assert_eq!(crisis.remaining_jornadas(), 0); // No cambia
    }
    
    // Test: Efectos
    #[test]
    fn test_crisis_effects() {
        let mut crisis = Crisis::new(CrisisId::from("c1"), CrisisSeverity::High, "".to_string());
        crisis.set_effect("polarization".to_string(), 10);
        crisis.set_effect("visibility".to_string(), -5);
        
        assert_eq!(crisis.get_effect("polarization"), Some(&10));
        assert_eq!(crisis.get_effect("visibility"), Some(&-5));
        assert_eq!(crisis.get_effect("nonexistent"), None);
    }
    
    // Test: Serializacion
    #[test]
    fn test_crisis_serialization() {
        let mut crisis = Crisis::new(CrisisId::from("c1"), CrisisSeverity::High, "Test".to_string());
        crisis.set_duration(3);
        
        let serialized = serde_json::to_string(&crisis).unwrap();
        let deserialized: Crisis = serde_json::from_str(&serialized).unwrap();
        assert_eq!(crisis.id(), deserialized.id());
        assert_eq!(crisis.severity(), deserialized.severity());
    }
}

