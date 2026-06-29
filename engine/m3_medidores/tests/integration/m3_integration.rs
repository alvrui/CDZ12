use m3_medidores::{MedidorError, MedidorId, MedidorManager, TipoMedidor};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_crear_medidor() {
        let mut manager = MedidorManager::new();
        manager.agregar_medidor(
            MedidorId::new("medidor_1".to_string()),
            "Test".to_string(),
            50,
            0,
            100,
            TipoMedidor::Progreso,
        );

        assert!(manager.obtener_medidor("medidor_1").is_ok());
    }

    #[test]
    fn test_manager_obtener_medidor_no_existente() {
        let manager = MedidorManager::new();
        assert!(manager.obtener_medidor("inexistente").is_err());
        assert_eq!(
            manager.obtener_medidor("inexistente").unwrap_err(),
            MedidorError::MedidorNoEncontrado("inexistente".to_string())
        );
    }

    #[test]
    fn test_manager_eliminar_medidor() {
        let mut manager = MedidorManager::new();
        manager.agregar_medidor(
            MedidorId::new("medidor_1".to_string()),
            "Test".to_string(),
            50,
            0,
            100,
            TipoMedidor::Progreso,
        );

        assert!(manager.eliminar_medidor("medidor_1").is_ok());
        assert!(manager.obtener_medidor("medidor_1").is_err());
    }

    #[test]
    fn test_manager_current_medidor() {
        let mut manager = MedidorManager::new();
        manager.agregar_medidor(
            MedidorId::new("medidor_1".to_string()),
            "Test".to_string(),
            50,
            0,
            100,
            TipoMedidor::Progreso,
        );

        assert!(manager.set_current_medidor("medidor_1").is_ok());
        assert!(manager.get_current_medidor().is_ok());
        assert_eq!(manager.get_current_medidor().unwrap().id.id, "medidor_1");
    }

    #[test]
    fn test_manager_operaciones_medidor() {
        let mut manager = MedidorManager::new();
        manager.agregar_medidor(
            MedidorId::new("medidor_1".to_string()),
            "Test".to_string(),
            50,
            0,
            100,
            TipoMedidor::Progreso,
        );

        let medidor = manager.obtener_medidor_mut("medidor_1").unwrap();
        assert!(medidor.aumentar(20).is_ok());
        assert_eq!(medidor.valor, 70);

        assert!(medidor.disminuir(30).is_ok());
        assert_eq!(medidor.valor, 40);
    }
}
