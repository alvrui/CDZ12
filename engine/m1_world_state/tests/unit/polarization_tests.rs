#[cfg(test)]
mod tests {
    use m1_world_state::polarization::{Polarization, PolarizationLevel};
    
    // Test: Creacion con valor inicial
    #[test]
    fn test_create_polarization() {
        let pol = Polarization::new(50);
        assert_eq!(pol.value(), 50);
        assert_eq!(pol.trend(), 0);
    }
    
    // Test: Valores limite
    #[test]
    fn test_polarization_bounds() {
        let pol_min = Polarization::new(0);
        let pol_max = Polarization::new(100);
        assert_eq!(pol_min.value(), 0);
        assert_eq!(pol_max.value(), 100);
    }
    
    // Test: Incremento y decremento
    #[test]
    fn test_polarization_change() {
        let mut pol = Polarization::new(50);
        pol.increase(10);
        assert_eq!(pol.value(), 60);
        pol.decrease(20);
        assert_eq!(pol.value(), 40);
    }
    
    // Test: Clamp en limites
    #[test]
    fn test_polarization_clamp() {
        let mut pol = Polarization::new(95);
        pol.increase(10);
        assert_eq!(pol.value(), 100); // Se clamp a 100
        
        let mut pol = Polarization::new(5);
        pol.decrease(10);
        assert_eq!(pol.value(), 0); // Se clamp a 0
    }
    
    // Test: Tendencias
    #[test]
    fn test_polarization_trend() {
        let mut pol = Polarization::new(50);
        pol.set_trend(2);
        pol.apply_trend();
        assert_eq!(pol.value(), 52);
        
        pol.set_trend(-3);
        pol.apply_trend();
        assert_eq!(pol.value(), 49);
    }
    
    // Test: Niveles de polarizacion
    #[test]
    fn test_polarization_levels() {
        assert!(matches!(Polarization::new(0).level(), PolarizationLevel::Minima));
        assert!(matches!(Polarization::new(25).level(), PolarizationLevel::Minima));
        assert!(matches!(Polarization::new(26).level(), PolarizationLevel::Baja));
        assert!(matches!(Polarization::new(50).level(), PolarizationLevel::Baja));
        assert!(matches!(Polarization::new(51).level(), PolarizationLevel::Media));
        assert!(matches!(Polarization::new(75).level(), PolarizationLevel::Media));
        assert!(matches!(Polarization::new(76).level(), PolarizationLevel::Alta));
        assert!(matches!(Polarization::new(90).level(), PolarizationLevel::Alta));
        assert!(matches!(Polarization::new(91).level(), PolarizationLevel::Maxima));
        assert!(matches!(Polarization::new(100).level(), PolarizationLevel::Maxima));
    }
    
    // Test: Serializacion
    #[test]
    fn test_polarization_serialization() {
        let mut pol = Polarization::new(50);
        pol.set_trend(1);
        let serialized = serde_json::to_string(&pol).unwrap();
        let deserialized: Polarization = serde_json::from_str(&serialized).unwrap();
        assert_eq!(pol.value(), deserialized.value());
        assert_eq!(pol.trend(), deserialized.trend());
    }
}
