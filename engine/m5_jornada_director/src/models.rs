use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TramoId {
    pub id: String,
}

impl TramoId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActoId {
    pub id: String,
}

impl ActoId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JornadaId {
    pub id: String,
}

impl JornadaId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tramo {
    pub id: TramoId,
    pub nombre: String,
    pub orden: u32,
    pub duracion: u32,
}

impl Tramo {
    pub fn new(id: TramoId, nombre: String, orden: u32, duracion: u32) -> Self {
        Self {
            id,
            nombre,
            orden,
            duracion,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Acto {
    pub id: ActoId,
    pub nombre: String,
    pub orden: u32,
    pub tramos: Vec<TramoId>,
}

impl Acto {
    pub fn new(id: ActoId, nombre: String, orden: u32, tramos: Vec<TramoId>) -> Self {
        Self {
            id,
            nombre,
            orden,
            tramos,
        }
    }

    pub fn add_tramo(&mut self, tramo_id: TramoId) {
        self.tramos.push(tramo_id);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Jornada {
    pub id: JornadaId,
    pub nombre: String,
    pub orden: u32,
    pub actos: Vec<ActoId>,
}

impl Jornada {
    pub fn new(id: JornadaId, nombre: String, orden: u32, actos: Vec<ActoId>) -> Self {
        Self {
            id,
            nombre,
            orden,
            actos,
        }
    }

    pub fn add_acto(&mut self, acto_id: ActoId) {
        self.actos.push(acto_id);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemporalState {
    pub jornada_actual: JornadaId,
    pub acto_actual: ActoId,
    pub tramo_actual: TramoId,
    pub tiempo_transcurrido: u32,
}

impl TemporalState {
    pub fn new(jornada: JornadaId, acto: ActoId, tramo: TramoId) -> Self {
        Self {
            jornada_actual: jornada,
            acto_actual: acto,
            tramo_actual: tramo,
            tiempo_transcurrido: 0,
        }
    }
}
