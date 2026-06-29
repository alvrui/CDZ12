use m3_medidores::{Medidor, MedidorId, Medidores, TipoMedidor};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_medidores() -> Medidores {
        let mut medidores = Medidores::new();
        medidores.medidores.insert(
            "medidor_1".to_string(),
            Medidor::new(
                MedidorId::new("medidor_1".to_string()),
                "Progreso".to_string(),
                50,
                0,
                100,
                TipoMedidor::Progreso,
            ),
        );
        medidores
    }

    #[test]
    fn test_serialize_to_json() {
        let medidores = create_medidores();
        let json = serde_json::to_string(&medidores).unwrap();
        assert!(json.contains("medidor_1"));
    }

    #[test]
    fn test_deserialize_from_json() {
        let medidores = create_medidores();
        let json = serde_json::to_string(&medidores).unwrap();
        let deserialized: Medidores = serde_json::from_str(&json).unwrap();
        assert_eq!(medidores.medidores.len(), deserialized.medidores.len());
    }

    #[test]
    fn test_serialize_to_yaml() {
        let medidores = create_medidores();
        let yaml = serde_yaml::to_string(&medidores).unwrap();
        assert!(yaml.contains("medidor_1"));
    }

    #[test]
    fn test_serialize_to_messagepack() {
        let medidores = create_medidores();
        let msgpack = rmp_serde::to_vec(&medidores).unwrap();
        assert!(!msgpack.is_empty());
    }
}
