use m5_jornada_director::{Acto, ActoId, Jornada, JornadaId, Tiempo, Tramo, TramoId};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tiempo() {
        let tiempo = Tiempo::new();
        assert!(tiempo.jornadas.is_empty());
        assert!(tiempo.actos.is_empty());
        assert!(tiempo.tramos.is_empty());
    }

    #[test]
    fn test_add_tramo() {
        let mut tiempo = Tiempo::new();
        let tramo = Tramo::new(
            TramoId::new("tramo_1".to_string()),
            "Mañana".to_string(),
            1,
            100,
        );
        tiempo.add_tramo(tramo);
        assert_eq!(tiempo.tramos.len(), 1);
    }

    #[test]
    fn test_add_acto() {
        let mut tiempo = Tiempo::new();
        let acto = Acto::new(
            ActoId::new("acto_1".to_string()),
            "Acto I".to_string(),
            1,
            vec![],
        );
        tiempo.add_acto(acto);
        assert_eq!(tiempo.actos.len(), 1);
    }

    #[test]
    fn test_add_jornada() {
        let mut tiempo = Tiempo::new();
        let jornada = Jornada::new(
            JornadaId::new("jornada_1".to_string()),
            "Día 1".to_string(),
            1,
            vec![],
        );
        tiempo.add_jornada(jornada);
        assert_eq!(tiempo.jornadas.len(), 1);
    }

    #[test]
    fn test_get_tramo() {
        let mut tiempo = Tiempo::new();
        let tramo_id = TramoId::new("tramo_1".to_string());
        let tramo = Tramo::new(tramo_id.clone(), "Mañana".to_string(), 1, 100);
        tiempo.add_tramo(tramo);

        let result = tiempo.get_tramo("tramo_1");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().nombre, "Mañana");
    }

    #[test]
    fn test_get_nonexistent_tramo() {
        let tiempo = Tiempo::new();
        let result = tiempo.get_tramo("tramo_1");
        assert!(result.is_err());
    }
}
