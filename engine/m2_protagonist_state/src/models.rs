use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtagonistId {
    pub id: String,
}

impl ProtagonistId {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}

/// Perfil del protagonista para selección de Story Elements
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Perfil {
    pub faccion: String,
    pub clase_social: String,
    pub profesion: String,
    pub ideologia: String,
}

impl Perfil {
    pub fn new(faccion: String, clase_social: String, profesion: String, ideologia: String) -> Self {
        Self {
            faccion,
            clase_social,
            profesion,
            ideologia,
        }
    }
}

/// Tendencia de los medidores (-1 = decreciente, 0 = estable, 1 = creciente)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tendencia {
    pub valor: i8,
}

impl Tendencia {
    pub fn new(valor: i8) -> Self {
        Self { valor }
    }
}

/// Medidores del protagonista (reemplazan a Attributes)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Medidores {
    pub fuerza: u8,
    pub influencia: u8,
    pub recursos: u8,
    pub reputacion: u8,
    pub lealtad: u8,
    pub salud: u8,
    pub tendencia: Tendencia,
}

impl Medidores {
    pub fn new(
        fuerza: u8,
        influencia: u8,
        recursos: u8,
        reputacion: u8,
        lealtad: u8,
        salud: u8,
        tendencia: Tendencia,
    ) -> Self {
        Self {
            fuerza,
            influencia,
            recursos,
            reputacion,
            lealtad,
            salud,
            tendencia,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Inventory {
    pub items: HashMap<String, InventoryItem>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InventoryItem {
    pub item_type: String,
    pub quantity: u32,
}

impl InventoryItem {
    pub fn new(item_type: String, quantity: u32) -> Self {
        Self {
            item_type,
            quantity,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relationships {
    pub factions: HashMap<String, i32>,
    pub npcs: HashMap<String, i32>,
}

impl Relationships {
    pub fn new() -> Self {
        Self {
            factions: HashMap::new(),
            npcs: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Objectives {
    pub list: HashMap<String, Objective>,
}

impl Objectives {
    pub fn new() -> Self {
        Self {
            list: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Objective {
    pub description: String,
    pub completed: bool,
    pub progress: u32,
}

impl Objective {
    pub fn new(description: String) -> Self {
        Self {
            description,
            completed: false,
            progress: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Protagonist {
    pub id: ProtagonistId,
    pub name: String,
    pub medidores: Medidores,
    pub inventory: Inventory,
    pub relationships: Relationships,
    pub objectives: Objectives,
    pub visibilidad: String,
    pub perfil: Perfil,
    pub etiquetas_protagonista: Vec<String>,
}

impl Protagonist {
    pub fn new(
        id: ProtagonistId,
        name: String,
        medidores: Medidores,
        inventory: Inventory,
        relationships: Relationships,
        objectives: Objectives,
        visibilidad: String,
        perfil: Perfil,
        etiquetas_protagonista: Vec<String>,
    ) -> Self {
        Self {
            id,
            name,
            medidores,
            inventory,
            relationships,
            objectives,
            visibilidad,
            perfil,
            etiquetas_protagonista,
        }
    }
}