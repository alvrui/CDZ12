use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::TiempoError;
use crate::models::{Acto, ActoId, Jornada, JornadaId, Tramo, TramoId};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tiempo {
    pub jornadas: HashMap<String, Jornada>,
    pub actos: HashMap<String, Acto>,
    pub tramos: HashMap<String, Tramo>,
}

impl Tiempo {
    pub fn new() -> Self {
        Self {
            jornadas: HashMap::new(),
            actos: HashMap::new(),
            tramos: HashMap::new(),
        }
    }

    pub fn add_jornada(&mut self, jornada: Jornada) {
        self.jornadas.insert(jornada.id.id.clone(), jornada);
    }

    pub fn add_acto(&mut self, acto: Acto) {
        self.actos.insert(acto.id.id.clone(), acto);
    }

    pub fn add_tramo(&mut self, tramo: Tramo) {
        self.tramos.insert(tramo.id.id.clone(), tramo);
    }

    pub fn get_jornada(&self, id: &str) -> Result<&Jornada, TiempoError> {
        self.jornadas
            .get(id)
            .ok_or_else(|| TiempoError::JornadaNoEncontrada(id.to_string()))
    }

    pub fn get_acto(&self, id: &str) -> Result<&Acto, TiempoError> {
        self.actos
            .get(id)
            .ok_or_else(|| TiempoError::ActoNoEncontrado(id.to_string()))
    }

    pub fn get_tramo(&self, id: &str) -> Result<&Tramo, TiempoError> {
        self.tramos
            .get(id)
            .ok_or_else(|| TiempoError::TramoNoEncontrado(id.to_string()))
    }
}
