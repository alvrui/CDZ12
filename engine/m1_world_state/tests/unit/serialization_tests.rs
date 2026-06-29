use m1_world_state::{
    models::WorldState,
    serialization::SerializationFormat,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_to_json() {
        let world = WorldState::new();
        let json = world.serialize(SerializationFormat::Json).unwrap();
        assert!(!json.is_empty());
    }

    #[test]
    fn test_deserialize_from_json() {
        let json = r#"{"time":{"tramo_id":1,"acto":"Prologo","jornada_absoluta":0},"polarization":{"value":0,"trend":0},"visibility":{"global_level":"Hidden","space_visibilities":{},"faction_visibilities":{}},"factions":[],"spaces":[],"active_crisis":null,"resolved_crises":[]}"#;
        let world: WorldState = WorldState::deserialize(json.as_bytes(), SerializationFormat::Json).unwrap();
        assert_eq!(world.time().jornada_absoluta(), 0);
    }

    #[test]
    fn test_serialize_to_yaml() {
        let world = WorldState::new();
        let yaml = world.serialize(SerializationFormat::Yaml).unwrap();
        assert!(!yaml.is_empty());
    }

    #[test]
    fn test_deserialize_from_yaml() {
        let yaml = "---\ntime:\n  tramo_id: 1\n  acto: Prologo\n  jornada_absoluta: 0\npolarization:\n  value: 0\n  trend: 0\nvisibility:\n  global_level: Hidden\n  space_visibilities: {}\n  faction_visibilities: {}\nfactions: []\nspaces: []\nactive_crisis: ~\nresolved_crises: []\n";
        let world: WorldState = WorldState::deserialize(yaml.as_bytes(), SerializationFormat::Yaml).unwrap();
        assert_eq!(world.time().jornada_absoluta(), 0);
    }

    #[test]
    fn test_serialize_to_messagepack() {
        let world = WorldState::new();
        let bytes = world.serialize(SerializationFormat::MessagePack).unwrap();
        assert!(!bytes.is_empty());
    }

    #[test]
    fn test_deserialize_from_messagepack() {
        let mut world = WorldState::new();
        world.advance_time(5);
        let bytes = world.serialize(SerializationFormat::MessagePack).unwrap();
        let deserialized = WorldState::deserialize(&bytes, SerializationFormat::MessagePack).unwrap();
        assert_eq!(world.time().jornada_absoluta(), deserialized.time().jornada_absoluta());
    }
}
