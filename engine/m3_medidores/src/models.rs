use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MedidorId {
    pub id: String,
}

impl MedidorId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TipoMedidor {
    Progreso,
    Recurso,
    Tiempo,
    Reputacion,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Medidor {
    pub id: MedidorId,
    pub nombre: String,
    pub valor: i32,
    pub min: i32,
    pub max: i32,
    pub tipo: TipoMedidor,
}

impl Medidor {
    pub fn new(
        id: MedidorId,
        nombre: String,
        valor_inicial: i32,
        min: i32,
        max: i32,
        tipo: TipoMedidor,
    ) -> Self {
        Self {
            id,
            nombre,
            valor: valor_inicial,
            min,
            max,
            tipo,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Medidores {
    pub medidores: HashMap<String, Medidor>,
}

impl Medidores {
    pub fn new() -> Self {
        Self {
            medidores: HashMap::new(),
        }
    }
}
