use m2_protagonist_state::{Attributes, ProtagonistError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_attributes() {
        let attrs = Attributes::new(50, 30, 20);
        assert_eq!(attrs.strength, 50);
        assert_eq!(attrs.influence, 30);
        assert_eq!(attrs.resources, 20);
    }

    #[test]
    fn test_increase_strength() {
        let mut attrs = Attributes::new(50, 30, 20);
        assert!(attrs.increase_strength(10).is_ok());
        assert_eq!(attrs.strength, 60);
    }

    #[test]
    fn test_decrease_strength() {
        let mut attrs = Attributes::new(50, 30, 20);
        assert!(attrs.decrease_strength(10).is_ok());
        assert_eq!(attrs.strength, 40);
    }

    #[test]
    fn test_strength_out_of_bounds_max() {
        let mut attrs = Attributes::new(95, 30, 20);
        assert!(attrs.increase_strength(10).is_err());
        assert_eq!(
            attrs.increase_strength(10).unwrap_err(),
            ProtagonistError::AttributeOutOfBounds("strength".to_string(), 0, 100)
        );
    }

    #[test]
    fn test_strength_out_of_bounds_min() {
        let mut attrs = Attributes::new(5, 30, 20);
        assert!(attrs.decrease_strength(10).is_err());
        assert_eq!(
            attrs.decrease_strength(10).unwrap_err(),
            ProtagonistError::AttributeOutOfBounds("strength".to_string(), 0, 100)
        );
    }

    #[test]
    fn test_influence_operations() {
        let mut attrs = Attributes::new(50, 30, 20);
        assert!(attrs.increase_influence(20).is_ok());
        assert_eq!(attrs.influence, 50);
        assert!(attrs.decrease_influence(10).is_ok());
        assert_eq!(attrs.influence, 40);
    }

    #[test]
    fn test_resources_operations() {
        let mut attrs = Attributes::new(50, 30, 20);
        assert!(attrs.increase_resources(30).is_ok());
        assert_eq!(attrs.resources, 50);
        assert!(attrs.decrease_resources(15).is_ok());
        assert_eq!(attrs.resources, 35);
    }
}
