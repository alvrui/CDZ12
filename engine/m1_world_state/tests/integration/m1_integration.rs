use m1_world_state::{
    WorldStateManager,  // <-- Trait necesario para usar los métodos
    entities::{Crisis, CrisisId, CrisisSeverity, SpaceId},
    models::WorldState,
    time::ActoNarrativo,
    visibility::{Visibility, VisibilityLevel, SpaceVisibility},
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_state_manager_trait() {
        let mut world = WorldState::new();
        world.advance_time(5);
        assert_eq!(world.get_jornada_absoluta(), 5);

        world.set_acto(ActoNarrativo::ActoI);
        assert!(matches!(world.get_acto(), ActoNarrativo::ActoI));
    }

    #[test]
    fn test_crisis_affect_world() {
        let mut world = WorldState::new();
        let mut crisis = Crisis::new(
            CrisisId::from("c1"),
            CrisisSeverity::High,
            "Test".to_string()
        );
        crisis.set_effect("polarization".to_string(), 15);

        world.set_active_crisis(Some(crisis));
        world.apply_crisis_effects();

        assert_eq!(world.polarization().value(), 65);
    }

    #[test]
    fn test_visibility_affects_access() {
        let mut world = WorldState::new();
        let mut visibility = Visibility::new(VisibilityLevel::Hidden);
        visibility.set_space_visibility(SpaceId::from("s1"), SpaceVisibility::Visible);
        world.set_visibility(visibility);

        assert!(!world.visibility().is_space_fully_visible(&SpaceId::from("s1")));
    }
}
