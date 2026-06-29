//! Entidad Espacio (genérica)
//!
//! Este módulo implementa la estructura de espacio de manera completamente
//! abstracta, sin referencias a localizaciones históricas concretas.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use super::faction::FactionId;

/// ID de espacio (wrapper para asegurar tipo fuerte)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpaceId(pub String);

impl SpaceId {
    /// Crea un nuevo ID de espacio
    pub fn from(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Obtiene el valor del ID
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SpaceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Tipo de espacio
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpaceType {
    /// Espacio público (accesible a todos)
    Public,
    /// Espacio privado (accesible a miembros)
    Private,
    /// Espacio secreto (accesible solo a iniciados)
    Secret,
}

/// Nivel de accesibilidad
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Accessibility {
    /// Completamente abierto
    Open,
    /// Acceso restringido
    Restricted,
    /// Completamente oculto
    Hidden,
}

/// Espacio genérico
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Space {
    /// ID único del espacio
    id: SpaceId,
    /// Tipo de espacio
    space_type: SpaceType,
    /// Nivel de accesibilidad
    accessibility: Accessibility,
    /// Nivel de peligrosidad (0-100)
    danger_level: u8,
    /// Espacios conectados
    connected_spaces: HashSet<SpaceId>,
    /// Facciones requeridas para acceder
    required_factions: HashSet<FactionId>,
    /// Medidores requeridos para acceder (nombre -> valor mínimo)
    required_medidores: HashMap<String, u8>,
}

impl Space {
    /// Crea un nuevo espacio con los parámetros especificados
    pub fn new(id: SpaceId, space_type: SpaceType, accessibility: Accessibility) -> Self {
        Self {
            id,
            space_type,
            accessibility,
            danger_level: 0,
            connected_spaces: HashSet::new(),
            required_factions: HashSet::new(),
            required_medidores: HashMap::new(),
        }
    }

    /// Obtiene el ID del espacio
    pub fn id(&self) -> &SpaceId {
        &self.id
    }

    /// Obtiene el tipo de espacio
    pub fn space_type(&self) -> SpaceType {
        self.space_type
    }

    /// Obtiene el nivel de accesibilidad
    pub fn accessibility(&self) -> Accessibility {
        self.accessibility
    }

    /// Obtiene el nivel de peligrosidad
    pub fn danger_level(&self) -> u8 {
        self.danger_level
    }

    /// Establece el nivel de peligrosidad (0-100)
    pub fn set_danger_level(&mut self, level: u8) {
        self.danger_level = level.clamp(0, 100);
    }

    /// Obtiene los espacios conectados
    pub fn connected_spaces(&self) -> &HashSet<SpaceId> {
        &self.connected_spaces
    }

    /// Añade una conexión a otro espacio
    pub fn add_connection(&mut self, other_id: SpaceId) {
        self.connected_spaces.insert(other_id);
    }

    /// Elimina una conexión
    pub fn remove_connection(&mut self, other_id: &SpaceId) {
        self.connected_spaces.remove(other_id);
    }

    /// Verifica si está conectado a otro espacio
    pub fn is_connected_to(&self, other_id: &SpaceId) -> bool {
        self.connected_spaces.contains(other_id)
    }

    /// Obtiene las facciones requeridas para acceder
    pub fn required_factions(&self) -> &HashSet<FactionId> {
        &self.required_factions
    }

    /// Añade una facción requerida
    pub fn add_required_faction(&mut self, faction_id: FactionId) {
        self.required_factions.insert(faction_id);
    }

    /// Elimina una facción requerida
    pub fn remove_required_faction(&mut self, faction_id: &FactionId) {
        self.required_factions.remove(faction_id);
    }

    /// Verifica si requiere una facción específica
    pub fn requires_faction(&self, faction_id: &FactionId) -> bool {
        self.required_factions.contains(faction_id)
    }

    /// Obtiene el valor mínimo requerido para un medidor
    pub fn required_medidor(&self, medidor_name: &str) -> Option<u8> {
        self.required_medidores.get(medidor_name).copied()
    }

    /// Añade un requerimiento de medidor
    pub fn add_required_medidor(&mut self, medidor_name: String, min_value: u8) {
        self.required_medidores.insert(medidor_name, min_value.clamp(0, 100));
    }

    /// Elimina un requerimiento de medidor
    pub fn remove_required_medidor(&mut self, medidor_name: &str) {
        self.required_medidores.remove(medidor_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::FactionId;

    #[test]
    fn test_space_creation() {
        let space = Space::new(
            SpaceId::from("espacio_1"),
            SpaceType::Public,
            Accessibility::Open
        );
        assert_eq!(space.id().as_str(), "espacio_1");
        assert!(matches!(space.space_type(), SpaceType::Public));
        assert!(matches!(space.accessibility(), Accessibility::Open));
    }

    #[test]
    fn test_space_connections() {
        let mut space1 = Space::new(SpaceId::from("s1"), SpaceType::Public, Accessibility::Open);
        let space2_id = SpaceId::from("s2");
        space1.add_connection(space2_id.clone());
        assert!(space1.is_connected_to(&space2_id));
    }

    #[test]
    fn test_space_requirements() {
        let mut space = Space::new(SpaceId::from("s1"), SpaceType::Private, Accessibility::Restricted);
        space.add_required_faction(FactionId::from("f1"));
        space.add_required_medidor("influencia".to_string(), 50);
        assert!(space.requires_faction(&FactionId::from("f1")));
        assert_eq!(space.required_medidor("influencia"), Some(50));
    }
}
