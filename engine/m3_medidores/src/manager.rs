use crate::error::MedidorError;
use crate::models::{Medidor, MedidorId, Medidores, TipoMedidor};

pub struct MedidorManager {
    medidores: Medidores,
    current_medidor_id: Option<String>,
}

impl MedidorManager {
    pub fn new() -> Self {
        Self {
            medidores: Medidores::new(),
            current_medidor_id: None,
        }
    }

    pub fn agregar_medidor(
        &mut self,
        id: MedidorId,
        nombre: String,
        valor_inicial: i32,
        min: i32,
        max: i32,
        tipo: TipoMedidor,
    ) {
        let medidor = Medidor::new(id.clone(), nombre, valor_inicial, min, max, tipo);
        self.medidores.medidores.insert(id.id, medidor);
    }

    pub fn obtener_medidor(&self, id: &str) -> Result<&Medidor, MedidorError> {
        self.medidores
            .medidores
            .get(id)
            .ok_or_else(|| MedidorError::MedidorNoEncontrado(id.to_string()))
    }

    pub fn obtener_medidor_mut(&mut self, id: &str) -> Result<&mut Medidor, MedidorError> {
        self.medidores
            .medidores
            .get_mut(id)
            .ok_or_else(|| MedidorError::MedidorNoEncontrado(id.to_string()))
    }

    pub fn eliminar_medidor(&mut self, id: &str) -> Result<Medidor, MedidorError> {
        self.medidores
            .medidores
            .remove(id)
            .ok_or_else(|| MedidorError::MedidorNoEncontrado(id.to_string()))
    }

    pub fn set_current_medidor(&mut self, id: &str) -> Result<(), MedidorError> {
        if self.medidores.medidores.contains_key(id) {
            self.current_medidor_id = Some(id.to_string());
            Ok(())
        } else {
            Err(MedidorError::MedidorNoEncontrado(id.to_string()))
        }
    }

    pub fn get_current_medidor(&self) -> Result<&Medidor, MedidorError> {
        self.current_medidor_id
            .as_deref()
            .and_then(|id| self.medidores.medidores.get(id))
            .ok_or_else(|| MedidorError::MedidorNoEncontrado("Ninguno seleccionado".to_string()))
    }
}
