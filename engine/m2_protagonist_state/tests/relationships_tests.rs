use m2_protagonist_state::{ProtagonistError, Relationships};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_relationships() {
        let rels = Relationships::new();
        assert!(rels.factions.is_empty());
    }

    #[test]
    fn test_add_relationship() {
        let mut rels = Relationships::new();
        rels.add_relationship("faction_1".to_string(), 50);
        assert_eq!(rels.factions.len(), 1);
        assert_eq!(rels.factions["faction_1"], 50);
    }

    #[test]
    fn test_update_relationship() {
        let mut rels = Relationships::new();
        rels.add_relationship("faction_1".to_string(), 50);
        assert!(rels.update_relationship("faction_1", 75).is_ok());
        assert_eq!(rels.factions["faction_1"], 75);
    }

    #[test]
    fn test_update_nonexistent_relationship() {
        let mut rels = Relationships::new();
        assert!(rels.update_relationship("faction_1", 75).is_err());
        assert_eq!(
            rels.update_relationship("faction_1", 75).unwrap_err(),
            ProtagonistError::RelationshipNotFound("faction_1".to_string())
        );
    }

    #[test]
    fn test_increase_relationship() {
        let mut rels = Relationships::new();
        rels.add_relationship("faction_1".to_string(), 50);
        assert!(rels.increase_relationship("faction_1", 10).is_ok());
        assert_eq!(rels.factions["faction_1"], 60);
    }

    #[test]
    fn test_get_relationship() {
        let mut rels = Relationships::new();
        rels.add_relationship("faction_1".to_string(), 50);
        assert!(rels.get_relationship("faction_1").is_ok());
        assert_eq!(rels.get_relationship("faction_1").unwrap(), 50);
    }

    #[test]
    fn test_remove_relationship() {
        let mut rels = Relationships::new();
        rels.add_relationship("faction_1".to_string(), 50);
        assert!(rels.remove_relationship("faction_1").is_ok());
        assert_eq!(rels.factions.len(), 0);
    }
}
