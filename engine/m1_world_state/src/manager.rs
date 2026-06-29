//! Manager del Estado del Mundo (M1)
//!
//! Este módulo implementa el trait WorldStateManager y su implementación
//! para gestionar el estado del mundo de juego.

use crate::{
    entities::{Crisis, CrisisId, Faction, FactionId, Space, SpaceId},
    models::WorldState,
    polarization::Polarization,
    time::{ActoNarrativo, TimeContext},
    visibility::Visibility,
};

/// Trait para gestionar el estado del mundo
pub trait WorldStateManager {
    /// Avanza el tiempo en el número de jornadas especificado
    fn advance_time(&mut self, delta: u32);

    /// Obtiene el contexto temporal actual
    fn get_current_time(&self) -> &TimeContext;

    /// Obtiene el ID del tramo actual
    fn get_tramo_id(&self) -> u32;

    /// Obtiene el acto narrativo actual
    fn get_acto(&self) -> ActoNarrativo;

    /// Obtiene la jornada absoluta actual
    fn get_jornada_absoluta(&self) -> u32;

    /// Establece el acto narrativo
    fn set_acto(&mut self, acto: ActoNarrativo);

    /// Obtiene la polarización actual
    fn get_polarization(&self) -> &Polarization;

    /// Obtiene el valor de polarización
    fn get_polarization_value(&self) -> u8;

    /// Establece la polarización
    fn set_polarization(&mut self, polarization: Polarization);

    /// Obtiene la visibilidad actual
    fn get_visibility(&self) -> &Visibility;

    /// Establece la visibilidad
    fn set_visibility(&mut self, visibility: Visibility);

    /// Obtiene todas las facciones
    fn get_factions(&self) -> &[Faction];

    /// Añade una facción
    fn add_faction(&mut self, faction: Faction);

    /// Obtiene una facción por ID
    fn get_faction(&self, id: &FactionId) -> Option<&Faction>;

    /// Obtiene todos los espacios
    fn get_spaces(&self) -> &[Space];

    /// Añade un espacio
    fn add_space(&mut self, space: Space);

    /// Obtiene un espacio por ID
    fn get_space(&self, id: &SpaceId) -> Option<&Space>;

    /// Obtiene la crisis activa
    fn get_active_crisis(&self) -> &Option<Crisis>;

    /// Establece la crisis activa
    fn set_active_crisis(&mut self, crisis: Option<Crisis>);

    /// Limpia la crisis activa
    fn clear_active_crisis(&mut self);

    /// Aplica los efectos de la crisis activa
    fn apply_crisis_effects(&mut self);
}

/// Implementación concreta de WorldStateManager
#[derive(Debug, Clone, PartialEq)]
pub struct WorldStateManagerImpl {
    state: WorldState,
}

impl WorldStateManagerImpl {
    /// Crea un nuevo manager con estado por defecto
    pub fn new() -> Self {
        Self {
            state: WorldState::new(),
        }
    }

    /// Crea un nuevo manager con configuración personalizada
    pub fn with_config(config: crate::models::WorldStateConfig) -> Self {
        Self {
            state: WorldState::with_config(config),
        }
    }

    /// Obtiene el estado del mundo
    pub fn state(&self) -> &WorldState {
        &self.state
    }

    /// Obtiene el estado del mundo mutable
    pub fn state_mut(&mut self) -> &mut WorldState {
        &mut self.state
    }

    /// Consume el manager y devuelve el estado
    pub fn into_state(self) -> WorldState {
        self.state
    }
}

impl Default for WorldStateManagerImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl WorldStateManager for WorldStateManagerImpl {
    fn advance_time(&mut self, delta: u32) {
        self.state.advance_time(delta);
    }

    fn get_current_time(&self) -> &TimeContext {
        self.state.time()
    }

    fn get_tramo_id(&self) -> u32 {
        self.state.time().tramo_id()
    }

    fn get_acto(&self) -> ActoNarrativo {
        self.state.time().acto()
    }

    fn get_jornada_absoluta(&self) -> u32 {
        self.state.time().jornada_absoluta()
    }

    fn set_acto(&mut self, acto: ActoNarrativo) {
        self.state.set_acto(acto);
    }

    fn get_polarization(&self) -> &Polarization {
        self.state.polarization()
    }

    fn get_polarization_value(&self) -> u8 {
        self.state.polarization().value()
    }

    fn set_polarization(&mut self, polarization: Polarization) {
        self.state.set_polarization(polarization);
    }

    fn get_visibility(&self) -> &Visibility {
        self.state.visibility()
    }

    fn set_visibility(&mut self, visibility: Visibility) {
        self.state.set_visibility(visibility);
    }

    fn get_factions(&self) -> &[Faction] {
        self.state.factions()
    }

    fn add_faction(&mut self, faction: Faction) {
        self.state.add_faction(faction);
    }

    fn get_faction(&self, id: &FactionId) -> Option<&Faction> {
        self.state.get_faction(id)
    }

    fn get_spaces(&self) -> &[Space] {
        self.state.spaces()
    }

    fn add_space(&mut self, space: Space) {
        self.state.add_space(space);
    }

    fn get_space(&self, id: &SpaceId) -> Option<&Space> {
        self.state.get_space(id)
    }

    fn get_active_crisis(&self) -> &Option<Crisis> {
        self.state.active_crisis()
    }

    fn set_active_crisis(&mut self, crisis: Option<Crisis>) {
        self.state.set_active_crisis(crisis);
    }

    fn clear_active_crisis(&mut self) {
        self.state.clear_active_crisis();
    }

    fn apply_crisis_effects(&mut self) {
        self.state.apply_crisis_effects();
    }
}

/// Implementación de WorldStateManager para WorldState directamente
impl WorldStateManager for WorldState {
    fn advance_time(&mut self, delta: u32) {
        self.advance_time(delta);
    }

    fn get_current_time(&self) -> &TimeContext {
        self.time()
    }

    fn get_tramo_id(&self) -> u32 {
        self.time().tramo_id()
    }

    fn get_acto(&self) -> ActoNarrativo {
        self.time().acto()
    }

    fn get_jornada_absoluta(&self) -> u32 {
        self.time().jornada_absoluta()
    }

    fn set_acto(&mut self, acto: ActoNarrativo) {
        self.set_acto(acto);
    }

    fn get_polarization(&self) -> &Polarization {
        self.polarization()
    }

    fn get_polarization_value(&self) -> u8 {
        self.polarization().value()
    }

    fn set_polarization(&mut self, polarization: Polarization) {
        self.set_polarization(polarization);
    }

    fn get_visibility(&self) -> &Visibility {
        self.visibility()
    }

    fn set_visibility(&mut self, visibility: Visibility) {
        self.set_visibility(visibility);
    }

    fn get_factions(&self) -> &[Faction] {
        self.factions()
    }

    fn add_faction(&mut self, faction: Faction) {
        self.add_faction(faction);
    }

    fn get_faction(&self, id: &FactionId) -> Option<&Faction> {
        self.get_faction(id)
    }

    fn get_spaces(&self) -> &[Space] {
        self.spaces()
    }

    fn add_space(&mut self, space: Space) {
        self.add_space(space);
    }

    fn get_space(&self, id: &SpaceId) -> Option<&Space> {
        self.get_space(id)
    }

    fn get_active_crisis(&self) -> &Option<Crisis> {
        self.active_crisis()
    }

    fn set_active_crisis(&mut self, crisis: Option<Crisis>) {
        self.set_active_crisis(crisis);
    }

    fn clear_active_crisis(&mut self) {
        self.clear_active_crisis();
    }

    fn apply_crisis_effects(&mut self) {
        self.apply_crisis_effects();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{polarization::Polarization, time::ActoNarrativo};

    #[test]
    fn test_manager_creation() {
        let manager = WorldStateManagerImpl::new();
        assert_eq!(manager.get_jornada_absoluta(), 0);
    }

    #[test]
    fn test_manager_advance_time() {
        let mut manager = WorldStateManagerImpl::new();
        manager.advance_time(5);
        assert_eq!(manager.get_jornada_absoluta(), 5);
    }

    #[test]
    fn test_manager_set_acto() {
        let mut manager = WorldStateManagerImpl::new();
        manager.set_acto(ActoNarrativo::ActoI);
        assert!(matches!(manager.get_acto(), ActoNarrativo::ActoI));
    }

    #[test]
    fn test_manager_polarization() {
        let mut manager = WorldStateManagerImpl::new();
        manager.set_polarization(Polarization::new(75));
        assert_eq!(manager.get_polarization_value(), 75);
    }
}
