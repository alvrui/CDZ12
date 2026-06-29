//! Contexto temporal del juego

use serde::{Deserialize, Serialize};
use std::fmt;

/// Actos narrativos del juego
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActoNarrativo {
    Prologo,
    ActoI,
    ActoII,
    ActoIII,
    Epilogo,
}

impl fmt::Display for ActoNarrativo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActoNarrativo::Prologo => write!(f, "Prologo"),
            ActoNarrativo::ActoI => write!(f, "ActoI"),
            ActoNarrativo::ActoII => write!(f, "ActoII"),
            ActoNarrativo::ActoIII => write!(f, "ActoIII"),
            ActoNarrativo::Epilogo => write!(f, "Epilogo"),
        }
    }
}

impl Default for ActoNarrativo {
    fn default() -> Self {
        Self::Prologo
    }
}

/// Contexto temporal del juego
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TimeContext {
    tramo_id: u32,
    acto: ActoNarrativo,
    jornada_absoluta: u32,
}

impl TimeContext {
    pub fn new(tramo_id: u32, acto: ActoNarrativo, jornada_absoluta: u32) -> Self {
        Self {
            tramo_id,
            acto,
            jornada_absoluta,
        }
    }
    
    pub fn tramo_id(&self) -> u32 {
        self.tramo_id
    }
    
    pub fn acto(&self) -> ActoNarrativo {
        self.acto
    }
    
    pub fn jornada_absoluta(&self) -> u32 {
        self.jornada_absoluta
    }
    
    pub fn set_tramo(&mut self, tramo_id: u32) {
        self.tramo_id = tramo_id;
    }
    
    pub fn set_acto(&mut self, acto: ActoNarrativo) {
        self.acto = acto;
    }
    
    pub fn advance_jornada(&mut self, count: u32) {
        self.jornada_absoluta += count;
    }
    
    pub fn advance_one_jornada(&mut self) {
        self.advance_jornada(1);
    }
    
    pub fn reset(&mut self) {
        self.jornada_absoluta = 0;
        self.tramo_id = 1;
        self.acto = ActoNarrativo::Prologo;
    }
}

impl Default for TimeContext {
    fn default() -> Self {
        Self::new(1, ActoNarrativo::default(), 0)
    }
}
