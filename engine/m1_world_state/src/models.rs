//! Estructuras de datos principales del modulo M1

use serde::{Deserialize, Serialize};

use crate::{
    entities::{Crisis, Faction, FactionId, Space, SpaceId},
    polarization::Polarization,
    time::{ActoNarrativo, TimeContext},
    visibility::{Visibility, VisibilityLevel},
};

/// Clima político del mundo
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClimaPolitico {
    Calma,
    Tension,
    Crisis,
    Revolucion,
}

impl Default for ClimaPolitico {
    fn default() -> Self {
        Self::Calma
    }
}

/// Estado completo del mundo de juego
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorldState {
    /// Contexto temporal del juego
    time: TimeContext,

    /// Nivel de polarizacion politica (0-100)
    polarization: Polarization,

    /// Visibilidad del tablero para el jugador
    visibility: Visibility,

    /// Facciones en el mundo (genericas, sin nombres historicos)
    factions: Vec<Faction>,

    /// Espacios en el mundo (genericos, sin nombres historicos)
    spaces: Vec<Space>,

    /// Crisis activa actual (si la hay)
    active_crisis: Option<Crisis>,

    /// Historial de crisis resueltas
    resolved_crises: Vec<Crisis>,

    /// Clima político actual
    clima_politico: ClimaPolitico,

    /// Etiquetas del mundo para selección de Story Elements
    etiquetas_mundo: Vec<String>,

    /// Eventos históricos ocurridos (para contexto narrativo)
    eventos_historicos_ocurridos: Vec<String>,
}

impl WorldState {
    /// Crea un nuevo estado del mundo con valores por defecto
    pub fn new() -> Self {
        Self {
            time: TimeContext::new(1, ActoNarrativo::Prologo, 0),
            polarization: Polarization::new(50),
            visibility: Visibility::new(VisibilityLevel::Partial),
            factions: Vec::new(),
            spaces: Vec::new(),
            active_crisis: None,
            resolved_crises: Vec::new(),
            clima_politico: ClimaPolitico::default(),
            etiquetas_mundo: Vec::new(),
            eventos_historicos_ocurridos: Vec::new(),
        }
    }

    /// Obtiene el contexto temporal
    pub fn time(&self) -> &TimeContext {
        &self.time
    }

    /// Obtiene el contexto temporal mutable
    pub fn time_mut(&mut self) -> &mut TimeContext {
        &mut self.time
    }

    /// Avanza el tiempo en el numero de jornadas especificado
    pub fn advance_time(&mut self, jornadas: u32) {
        self.time.advance_jornada(jornadas);
    }

    /// Establece el acto narrativo actual
    pub fn set_acto(&mut self, acto: ActoNarrativo) {
        self.time.set_acto(acto);
    }

    /// Obtiene la polarizacion
    pub fn polarization(&self) -> &Polarization {
        &self.polarization
    }

    /// Obtiene la polarizacion mutable
    pub fn polarization_mut(&mut self) -> &mut Polarization {
        &mut self.polarization
    }

    /// Establece la polarizacion
    pub fn set_polarization(&mut self, polarization: Polarization) {
        self.polarization = polarization;
    }

    /// Obtiene la visibilidad
    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    /// Obtiene la visibilidad mutable
    pub fn visibility_mut(&mut self) -> &mut Visibility {
        &mut self.visibility
    }

    /// Establece la visibilidad
    pub fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility = visibility;
    }

    /// Obtiene las facciones
    pub fn factions(&self) -> &[Faction] {
        &self.factions
    }

    /// Anade una faccion
    pub fn add_faction(&mut self, faction: Faction) {
        self.factions.push(faction);
    }

    /// Obtiene una faccion por ID
    pub fn get_faction(&self, id: &FactionId) -> Option<&Faction> {
        self.factions.iter().find(|f| f.id() == id)
    }

    /// Obtiene las facciones mutable
    pub fn factions_mut(&mut self) -> &mut Vec<Faction> {
        &mut self.factions
    }

    /// Obtiene los espacios
    pub fn spaces(&self) -> &[Space] {
        &self.spaces
    }

    /// Anade un espacio
    pub fn add_space(&mut self, space: Space) {
        self.spaces.push(space);
    }

    /// Obtiene un espacio por ID
    pub fn get_space(&self, id: &SpaceId) -> Option<&Space> {
        self.spaces.iter().find(|s| s.id() == id)
    }

    /// Obtiene la crisis activa
    pub fn active_crisis(&self) -> &Option<Crisis> {
        &self.active_crisis
    }

    /// Establece la crisis activa
    pub fn set_active_crisis(&mut self, crisis: Option<Crisis>) {
        self.active_crisis = crisis;
    }

    /// Limpia la crisis activa
    pub fn clear_active_crisis(&mut self) {
        self.active_crisis = None;
    }

    /// Obtiene las crisis resueltas
    pub fn resolved_crises(&self) -> &[Crisis] {
        &self.resolved_crises
    }

    /// Anade una crisis resuelta
    pub fn add_resolved_crisis(&mut self, crisis: Crisis) {
        self.resolved_crises.push(crisis);
    }

    /// Obtiene el clima político
    pub fn clima_politico(&self) -> &ClimaPolitico {
        &self.clima_politico
    }

    /// Establece el clima político
    pub fn set_clima_politico(&mut self, clima: ClimaPolitico) {
        self.clima_politico = clima;
    }

    /// Obtiene las etiquetas del mundo
    pub fn etiquetas_mundo(&self) -> &[String] {
        &self.etiquetas_mundo
    }

    /// Establece las etiquetas del mundo
    pub fn set_etiquetas_mundo(&mut self, etiquetas: Vec<String>) {
        self.etiquetas_mundo = etiquetas;
    }

    /// Obtiene los eventos históricos ocurridos
    pub fn eventos_historicos_ocurridos(&self) -> &[String] {
        &self.eventos_historicos_ocurridos
    }

    /// Anade un evento histórico ocurrido
    pub fn add_evento_historico(&mut self, evento: String) {
        self.eventos_historicos_ocurridos.push(evento);
    }

    /// Aplica los efectos de la crisis activa
    pub fn apply_crisis_effects(&mut self) {
        if let Some(crisis) = self.active_crisis.clone() {
            for (property, effect) in crisis.effects() {
                match property.as_str() {
                    "polarization" => {
                        let mut pol = self.polarization().clone();
                        pol.increase(*effect as u8);
                        self.set_polarization(pol);
                    }
                    _ => {
                        // Otros efectos se manejaran en futuras versiones
                    }
                }
            }
        }
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuracion para crear un nuevo WorldState
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldStateConfig {
    pub initial_tramo: u32,
    pub initial_acto: ActoNarrativo,
    pub initial_jornada: u32,
    pub initial_polarization: u8,
    pub initial_visibility: VisibilityLevel,
}

impl Default for WorldStateConfig {
    fn default() -> Self {
        Self {
            initial_tramo: 1,
            initial_acto: ActoNarrativo::Prologo,
            initial_jornada: 0,
            initial_polarization: 50,
            initial_visibility: VisibilityLevel::Partial,
        }
    }
}

impl WorldState {
    /// Crea un nuevo WorldState desde una configuracion
    pub fn with_config(config: WorldStateConfig) -> Self {
        Self {
            time: TimeContext::new(
                config.initial_tramo,
                config.initial_acto,
                config.initial_jornada,
            ),
            polarization: Polarization::new(config.initial_polarization),
            visibility: Visibility::new(config.initial_visibility),
            factions: Vec::new(),
            spaces: Vec::new(),
            active_crisis: None,
            resolved_crises: Vec::new(),
            clima_politico: ClimaPolitico::default(),
            etiquetas_mundo: Vec::new(),
            eventos_historicos_ocurridos: Vec::new(),
        }
    }
}