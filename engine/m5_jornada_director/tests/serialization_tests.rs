use m5_jornada_director::{Jornada, JornadaId, Tiempo, Acto, ActoId, Tramo, TramoId};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tiempo() -> Tiempo {
        let mut tiempo = Tiempo::new();

        let tramo = Tramo::new(
            TramoId::new("tramo_1".to_string()),
            "Mañana".to_string(),
            1,
            100,
        );
        tiempo.add_tramo(tramo);

        let acto = Acto::new(
            ActoId::new("acto_1".to_string()),
            "Acto I".to_string(),
            1,
            vec![TramoId::new("tramo_1".to_string())],
        );
        tiempo.add_acto(acto);

        let jornada = Jornada::new(
            JornadaId::new("jornada_1".to_string()),
            "Día 1".to_string(),
            1,
            vec![ActoId::new("acto_1".to_string())],
        );
        tiempo.add_jornada(jornada);

        tiempo
    }

    #[test]
    fn test_serialize_to_json() {
        let tiempo = create_test_tiempo();
        let json = serde_json::to_string(&tiempo).unwrap();
        assert!(json.contains("jornada_1"));
    }

    #[test]
    fn test_deserialize_from_json() {
        let tiempo = create_test_tiempo();
        let json = serde_json::to_string(&tiempo).unwrap();
        let deserialized: Tiempo = serde_json::from_str(&json).unwrap();
        assert_eq!(tiempo.jornadas.len(), deserialized.jornadas.len());
    }

    #[test]
    fn test_serialize_to_yaml() {
        let tiempo = create_test_tiempo();
        let yaml = serde_yaml::to_string(&tiempo).unwrap();
        assert!(yaml.contains("jornada_1"));
    }

    #[test]
    fn test_serialize_to_messagepack() {
        let tiempo = create_test_tiempo();
        let msgpack = rmp_serde::to_vec(&tiempo).unwrap();
        assert!(!msgpack.is_empty());
    }
}
