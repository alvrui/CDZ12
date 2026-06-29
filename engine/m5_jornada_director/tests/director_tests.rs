use m5_jornada_director::{Acto, ActoId, Jornada, JornadaId, JornadaDirector, Tiempo, TemporalState, Tramo, TramoId, TiempoError};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_tiempo() -> Tiempo {
        let mut tiempo = Tiempo::new();

        // Crear tramos
        let tramo1 = Tramo::new(TramoId::new("tramo_1".to_string()), "Mañana".to_string(), 1, 100);
        let tramo2 = Tramo::new(TramoId::new("tramo_2".to_string()), "Tarde".to_string(), 2, 100);
        let tramo3 = Tramo::new(TramoId::new("tramo_3".to_string()), "Noche".to_string(), 3, 100);

        tiempo.add_tramo(tramo1);
        tiempo.add_tramo(tramo2);
        tiempo.add_tramo(tramo3);

        // Crear actos
        let mut acto1 = Acto::new(ActoId::new("acto_1".to_string()), "Acto I".to_string(), 1, vec![]);
        acto1.add_tramo(TramoId::new("tramo_1".to_string()));
        acto1.add_tramo(TramoId::new("tramo_2".to_string()));

        let mut acto2 = Acto::new(ActoId::new("acto_2".to_string()), "Acto II".to_string(), 2, vec![]);
        acto2.add_tramo(TramoId::new("tramo_3".to_string()));

        tiempo.add_acto(acto1);
        tiempo.add_acto(acto2);

        // Crear jornadas
        let mut jornada1 = Jornada::new(JornadaId::new("jornada_1".to_string()), "Día 1".to_string(), 1, vec![]);
        jornada1.add_acto(ActoId::new("acto_1".to_string()));
        jornada1.add_acto(ActoId::new("acto_2".to_string()));

        tiempo.add_jornada(jornada1);

        tiempo
    }

    #[test]
    fn test_create_director() {
        let tiempo = create_test_tiempo();
        let state = TemporalState::new(
            JornadaId::new("jornada_1".to_string()),
            ActoId::new("acto_1".to_string()),
            TramoId::new("tramo_1".to_string()),
        );
        let director = JornadaDirector::new(tiempo, state);
        assert_eq!(director.get_state().jornada_actual.id, "jornada_1");
    }

    #[test]
    fn test_advance_tramo() {
        let tiempo = create_test_tiempo();
        let state = TemporalState::new(
            JornadaId::new("jornada_1".to_string()),
            ActoId::new("acto_1".to_string()),
            TramoId::new("tramo_1".to_string()),
        );
        let mut director = JornadaDirector::new(tiempo, state);

        assert!(director.advance_tramo().is_ok());
        assert_eq!(director.get_state().tramo_actual.id, "tramo_2");
    }

    #[test]
    fn test_advance_tramo_no_mas() {
        let tiempo = create_test_tiempo();
        let state = TemporalState::new(
            JornadaId::new("jornada_1".to_string()),
            ActoId::new("acto_1".to_string()),
            TramoId::new("tramo_2".to_string()),
        );
        let mut director = JornadaDirector::new(tiempo, state);

        let result = director.advance_tramo();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TiempoError::NoMasTramosEnActo);
    }

    #[test]
    fn test_advance_acto() {
        let tiempo = create_test_tiempo();
        let state = TemporalState::new(
            JornadaId::new("jornada_1".to_string()),
            ActoId::new("acto_1".to_string()),
            TramoId::new("tramo_1".to_string()),
        );
        let mut director = JornadaDirector::new(tiempo, state);

        assert!(director.advance_acto().is_ok());
        assert_eq!(director.get_state().acto_actual.id, "acto_2");
        assert_eq!(director.get_state().tramo_actual.id, "tramo_3");
    }

    #[test]
    fn test_increment_time() {
        let tiempo = create_test_tiempo();
        let state = TemporalState::new(
            JornadaId::new("jornada_1".to_string()),
            ActoId::new("acto_1".to_string()),
            TramoId::new("tramo_1".to_string()),
        );
        let mut director = JornadaDirector::new(tiempo, state);

        assert!(director.increment_time(50).is_ok());
        assert_eq!(director.get_state().tiempo_transcurrido, 50);
    }

    #[test]
    fn test_increment_time_exceeds_duration() {
        let tiempo = create_test_tiempo();
        let state = TemporalState::new(
            JornadaId::new("jornada_1".to_string()),
            ActoId::new("acto_1".to_string()),
            TramoId::new("tramo_1".to_string()),
        );
        let mut director = JornadaDirector::new(tiempo, state);

        let result = director.increment_time(150);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), TiempoError::NoMasTramosEnActo);
    }
}
