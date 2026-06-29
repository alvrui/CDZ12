#[cfg(test)]
mod tests {
    use m1_world_state::time::{TimeContext, ActoNarrativo};
    
    // Test: Creacion de TimeContext valido
    #[test]
    fn test_create_valid_time_context() {
        let time = TimeContext::new(1, ActoNarrativo::Prologo, 5);
        assert_eq!(time.tramo_id(), 1);
        assert!(matches!(time.acto(), ActoNarrativo::Prologo));
        assert_eq!(time.jornada_absoluta(), 5);
    }
    
    // Test: Avance de jornada
    #[test]
    fn test_advance_jornada() {
        let mut time = TimeContext::new(1, ActoNarrativo::Prologo, 5);
        time.advance_jornada(3);
        assert_eq!(time.jornada_absoluta(), 8);
    }
    
    // Test: Cambio de acto
    #[test]
    fn test_change_acto() {
        let mut time = TimeContext::new(1, ActoNarrativo::Prologo, 5);
        time.set_acto(ActoNarrativo::ActoI);
        assert!(matches!(time.acto(), ActoNarrativo::ActoI));
    }
    
    // Test: Cambio de tramo
    #[test]
    fn test_change_tramo() {
        let mut time = TimeContext::new(1, ActoNarrativo::Prologo, 5);
        time.set_tramo(2);
        assert_eq!(time.tramo_id(), 2);
    }
    
    // Test: Serializacion
    #[test]
    fn test_time_context_serialization() {
        let time = TimeContext::new(1, ActoNarrativo::ActoI, 10);
        let serialized = serde_json::to_string(&time).unwrap();
        let deserialized: TimeContext = serde_json::from_str(&serialized).unwrap();
        assert_eq!(time, deserialized);
    }
}
