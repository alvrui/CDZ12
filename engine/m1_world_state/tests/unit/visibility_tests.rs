#[cfg(test)]
mod tests {
    use m1_world_state::visibility::{Visibility, VisibilityLevel, SpaceVisibility};
    use m1_world_state::entities::{SpaceId, FactionId};

    #[test]
    fn test_visibility_creation() {
        let vis = Visibility::new(VisibilityLevel::Partial);
        assert!(matches!(vis.global_level(), VisibilityLevel::Partial));
    }

    #[test]
    fn test_space_visibility() {
        let mut vis = Visibility::new(VisibilityLevel::Hidden);
        vis.set_space_visibility(SpaceId::from("space_1"), SpaceVisibility::Visible);
        vis.set_space_visibility(SpaceId::from("space_2"), SpaceVisibility::Hidden);

        assert!(matches!(
            vis.get_space_visibility(&SpaceId::from("space_1")),
            Some(SpaceVisibility::Visible)
        ));
        assert!(matches!(
            vis.get_space_visibility(&SpaceId::from("space_2")),
            Some(SpaceVisibility::Hidden)
        ));
    }

    #[test]
    fn test_faction_visibility() {
        let mut vis = Visibility::new(VisibilityLevel::Partial);
        let mut faction_vis = std::collections::HashMap::new();
        faction_vis.insert(FactionId::from("faccion_1"), VisibilityLevel::Full);
        faction_vis.insert(FactionId::from("faccion_2"), VisibilityLevel::Hidden);
        vis.set_faction_visibilities(faction_vis);

        assert!(matches!(
            vis.get_faction_visibility(&FactionId::from("faccion_1")),
            Some(VisibilityLevel::Full)
        ));
    }

    #[test]
    fn test_effective_visibility() {
        let mut vis = Visibility::new(VisibilityLevel::Full);
        vis.set_space_visibility(SpaceId::from("space_1"), SpaceVisibility::Visible);
        assert!(matches!(
            vis.effective_space_visibility(&SpaceId::from("space_1")),
            VisibilityLevel::Full
        ));
    }

    #[test]
    fn test_hidden_global_override() {
        let mut vis = Visibility::new(VisibilityLevel::Hidden);
        vis.set_space_visibility(SpaceId::from("space_1"), SpaceVisibility::Visible);
        assert!(matches!(
            vis.effective_space_visibility(&SpaceId::from("space_1")),
            VisibilityLevel::Hidden
        ));
    }

    #[test]
    fn test_visibility_serialization() {
        let mut vis = Visibility::new(VisibilityLevel::Partial);
        vis.set_space_visibility(SpaceId::from("space_1"), SpaceVisibility::Visible);
        let serialized = serde_json::to_string(&vis).unwrap();
        let deserialized: Visibility = serde_json::from_str(&serialized).unwrap();
        assert_eq!(vis.global_level(), deserialized.global_level());
    }
}
