use m3_medidores::{Medidor, MedidorError, MedidorId, TipoMedidor};

#[cfg(test)]
mod tests {
    use super::*;

    fn create_medidor() -> Medidor {
        Medidor::new(
            MedidorId::new("medidor_1".to_string()),
            "Progreso misión".to_string(),
            50,
            0,
            100,
            TipoMedidor::Progreso,
        )
    }

    #[test]
    fn test_crear_medidor() {
        let medidor = create_medidor();
        assert_eq!(medidor.id.id, "medidor_1");
        assert_eq!(medidor.nombre, "Progreso misión");
        assert_eq!(medidor.valor, 50);
        assert_eq!(medidor.min, 0);
        assert_eq!(medidor.max, 100);
    }

    #[test]
    fn test_aumentar_medidor() {
        let mut medidor = create_medidor();
        assert!(medidor.aumentar(10).is_ok());
        assert_eq!(medidor.valor, 60);
    }

    #[test]
    fn test_disminuir_medidor() {
        let mut medidor = create_medidor();
        assert!(medidor.disminuir(20).is_ok());
        assert_eq!(medidor.valor, 30);
    }

    #[test]
    fn test_medidor_fuera_de_limites_max() {
        let mut medidor = create_medidor();
        let result = medidor.aumentar(60);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MedidorError::ValorFueraDeLimites("medidor_1".to_string(), 0, 100, 110)
        );
        // Verificar que el valor NO se modificó
        assert_eq!(medidor.valor, 50);
    }

    #[test]
    fn test_medidor_fuera_de_limites_min() {
        let mut medidor = create_medidor();
        let result = medidor.disminuir(60);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            MedidorError::ValorFueraDeLimites("medidor_1".to_string(), 0, 100, -10)
        );
        // Verificar que el valor NO se modificó
        assert_eq!(medidor.valor, 50);
    }

    #[test]
    fn test_resetear_medidor() {
        let mut medidor = create_medidor();
        assert!(medidor.disminuir(20).is_ok());
        assert!(medidor.resetear().is_ok());
        assert_eq!(medidor.valor, 0);
    }

    #[test]
    fn test_porcentajear() {
        let medidor = create_medidor();
        assert_eq!(medidor.porcentajear(), 50.0);
    }

    #[test]
    fn test_porcentajear_cero() {
        let medidor = Medidor::new(
            MedidorId::new("medidor_2".to_string()),
            "Vacío".to_string(),
            0,
            0,
            100,
            TipoMedidor::Recurso,
        );
        assert_eq!(medidor.porcentajear(), 0.0);
    }
}
