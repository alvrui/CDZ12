//! Sistema de visibilidad del tablero
//!
//! Este módulo implementa la gestión de qué información está visible
//! para el jugador, de manera completamente abstracta.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::entities::{SpaceId, FactionId};

/// Niveles de visibilidad global
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VisibilityLevel {
    /// Nada es visible
    Hidden,
    /// Solo información básica es visible
    Partial,
    /// Todo es visible
    Full,
}

/// Niveles de visibilidad para espacios específicos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpaceVisibility {
    /// Completamente visible
    Visible,
    /// Oculto
    Hidden,
    /// Niebla de guerra (parcialmente visible)
    FogOfWar,
}

/// Visibilidad del tablero para el jugador
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Visibility {
    /// Nivel de visibilidad global
    global_level: VisibilityLevel,
    /// Visibilidad por espacio (override del nivel global)
    space_visibilities: HashMap<SpaceId, SpaceVisibility>,
    /// Visibilidad por facción
    faction_visibilities: HashMap<FactionId, VisibilityLevel>,
}

impl Visibility {
    /// Crea una nueva visibilidad con el nivel global especificado
    pub fn new(global_level: VisibilityLevel) -> Self {
        Self {
            global_level,
            space_visibilities: HashMap::new(),
            faction_visibilities: HashMap::new(),
        }
    }

    /// Obtiene el nivel de visibilidad global
    pub fn global_level(&self) -> VisibilityLevel {
        self.global_level
    }

    /// Establece el nivel de visibilidad global
    pub fn set_global_level(&mut self, level: VisibilityLevel) {
        self.global_level = level;
    }

    /// Obtiene la visibilidad de un espacio específico
    pub fn get_space_visibility(&self, space_id: &SpaceId) -> Option<SpaceVisibility> {
        self.space_visibilities.get(space_id).copied()
    }

    /// Establece la visibilidad de un espacio
    pub fn set_space_visibility(&mut self, space_id: SpaceId, visibility: SpaceVisibility) {
        self.space_visibilities.insert(space_id, visibility);
    }

    /// Obtiene la visibilidad de una facción específica
    pub fn get_faction_visibility(&self, faction_id: &FactionId) -> Option<VisibilityLevel> {
        self.faction_visibilities.get(faction_id).copied()
    }

    /// Establece la visibilidad de una facción
    pub fn set_faction_visibility(&mut self, faction_id: FactionId, visibility: VisibilityLevel) {
        self.faction_visibilities.insert(faction_id, visibility);
    }

    /// Establece visibilidad para múltiples facciones
    pub fn set_faction_visibilities(&mut self, visibilities: HashMap<FactionId, VisibilityLevel>) {
        self.faction_visibilities = visibilities;
    }

    /// Calcula la visibilidad efectiva para un espacio
    pub fn effective_space_visibility(&self, space_id: &SpaceId) -> VisibilityLevel {
        if matches!(self.global_level, VisibilityLevel::Hidden) {
            return VisibilityLevel::Hidden;
        }

        match self.get_space_visibility(space_id) {
            Some(SpaceVisibility::Visible) => VisibilityLevel::Full,
            Some(SpaceVisibility::Hidden) => VisibilityLevel::Hidden,
            Some(SpaceVisibility::FogOfWar) | None => self.global_level,
        }
    }

    /// Verifica si un espacio es completamente visible
    pub fn is_space_fully_visible(&self, space_id: &SpaceId) -> bool {
        matches!(
            self.effective_space_visibility(space_id),
            VisibilityLevel::Full
        )
    }

    /// Verifica si una facción es visible
    pub fn is_faction_visible(&self, faction_id: &FactionId) -> bool {
        match self.get_faction_visibility(faction_id) {
            Some(level) => !matches!(level, VisibilityLevel::Hidden),
            None => !matches!(self.global_level, VisibilityLevel::Hidden),
        }
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Self::new(VisibilityLevel::Partial)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{SpaceId, FactionId};

    #[test]
    fn test_visibility_creation() {
        let vis = Visibility::new(VisibilityLevel::Partial);
        assert!(matches!(vis.global_level(), VisibilityLevel::Partial));
    }

    #[test]
    fn test_space_visibility() {
        let mut vis = Visibility::new(VisibilityLevel::Hidden);
        vis.set_space_visibility(SpaceId::from("space_1"), SpaceVisibility::Visible);
        assert!(matches!(
            vis.get_space_visibility(&SpaceId::from("space_1")),
            Some(SpaceVisibility::Visible)
        ));
    }

    #[test]
    fn test_effective_visibility() {
        let mut vis = Visibility::new(VisibilityLevel::Full);
        vis.set_space_visibility(SpaceId::from("space_1"), SpaceVisibility::Visible);
        assert!(matches!(
            vis.effective_space_visibility(&SpaceId::from("space_1")),
            VisibilityLevel::Full
        ));
    }

    #[test]
    fn test_hidden_global_override() {
        let mut vis = Visibility::new(VisibilityLevel::Hidden);
        vis.set_space_visibility(SpaceId::from("space_1"), SpaceVisibility::Visible);
        assert!(matches!(
            vis.effective_space_visibility(&SpaceId::from("space_1")),
            VisibilityLevel::Hidden
        ));
    }
}
