//! Sistema de polarizacion politica abstracta

use serde::{Deserialize, Serialize};

/// Niveles de polarizacion para clasificacion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PolarizationLevel {
    Minima,
    Baja,
    Media,
    Alta,
    Maxima,
}

/// Valor de polarizacion (0-100)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolarizationValue(u8);

impl PolarizationValue {
    pub const MIN: u8 = 0;
    pub const MAX: u8 = 100;
    
    pub fn new(value: u8) -> Self {
        Self(value.clamp(Self::MIN, Self::MAX))
    }
    
    pub fn value(&self) -> u8 {
        self.0
    }
    
    pub fn increase(&mut self, delta: i8) {
        let new_value = (self.0 as i16) + (delta as i16);
        self.0 = new_value.clamp(Self::MIN as i16, Self::MAX as i16) as u8;
    }
    
    pub fn decrease(&mut self, delta: i8) {
        self.increase(-delta);
    }
}

impl Default for PolarizationValue {
    fn default() -> Self {
        Self(50)
    }
}

/// Polarizacion politica
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polarization {
    value: PolarizationValue,
    trend: i8,
}

impl Polarization {
    pub fn new(value: u8) -> Self {
        Self {
            value: PolarizationValue::new(value),
            trend: 0,
        }
    }
    
    pub fn value(&self) -> u8 {
        self.value.value()
    }
    
    pub fn trend(&self) -> i8 {
        self.trend
    }
    
    pub fn set_trend(&mut self, trend: i8) {
        self.trend = trend.clamp(-3, 3);
    }
    
    pub fn increase(&mut self, delta: u8) {
        self.value.increase(delta as i8);
    }
    
    pub fn decrease(&mut self, delta: u8) {
        self.value.decrease(delta as i8);
    }
    
    pub fn apply_trend(&mut self) {
        self.value.increase(self.trend);
    }
    
    pub fn level(&self) -> PolarizationLevel {
        match self.value.value() {
            0..=25 => PolarizationLevel::Minima,
            26..=50 => PolarizationLevel::Baja,
            51..=75 => PolarizationLevel::Media,
            76..=90 => PolarizationLevel::Alta,
            91..=100 => PolarizationLevel::Maxima,
            _ => unreachable!(),
        }
    }
    
    pub fn reset(&mut self) {
        self.value = PolarizationValue::default();
        self.trend = 0;
    }
}

impl Default for Polarization {
    fn default() -> Self {
        Self::new(50)
    }
}
