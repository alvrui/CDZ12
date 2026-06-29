//! Entidad Crisis
//!
//! Este módulo implementa la estructura de crisis de manera completamente
//! abstracta, sin referencias a eventos históricos concretos.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ID de crisis (wrapper para asegurar tipo fuerte)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CrisisId(pub String);

impl CrisisId {
    /// Crea un nuevo ID de crisis
    pub fn from(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Obtiene el valor del ID
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CrisisId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Severidad de la crisis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CrisisSeverity {
    /// Baja severidad
    Low,
    /// Severidad media
    Medium,
    /// Alta severidad
    High,
    /// Severidad crítica
    Critical,
}

/// Fase de la crisis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CrisisPhase {
    /// Pendiente de activación
    Pending,
    /// Activa
    Active,
    /// En resolución
    Resolving,
    /// Resuelta
    Resolved,
}

/// Crisis genérica
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Crisis {
    /// ID único de la crisis
    id: CrisisId,
    /// Severidad de la crisis
    severity: CrisisSeverity,
    /// Descripción abstracta (sin contenido histórico)
    description: String,
    /// Fase actual de la crisis
    current_phase: CrisisPhase,
    /// Duración total en jornadas (0 = permanente)
    duration: u32,
    /// Jornadas restantes (0 = permanente)
    remaining_jornadas: u32,
    /// Si la crisis es permanente
    is_permanent: bool,
    /// Efectos de la crisis (propiedad -> valor de cambio)
    effects: HashMap<String, i32>,
}

impl Crisis {
    /// Crea una nueva crisis con los parámetros especificados
    pub fn new(id: CrisisId, severity: CrisisSeverity, description: String) -> Self {
        Self {
            id,
            severity,
            description,
            current_phase: CrisisPhase::Pending,
            duration: 0,
            remaining_jornadas: 0,
            is_permanent: true,
            effects: HashMap::new(),
        }
    }

    /// Obtiene el ID de la crisis
    pub fn id(&self) -> &CrisisId {
        &self.id
    }

    /// Obtiene la severidad
    pub fn severity(&self) -> CrisisSeverity {
        self.severity
    }

    /// Obtiene la descripción
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Obtiene la fase actual
    pub fn current_phase(&self) -> CrisisPhase {
        self.current_phase
    }

    /// Avanza a la siguiente fase
    pub fn advance_phase(&mut self) {
        self.current_phase = match self.current_phase {
            CrisisPhase::Pending => CrisisPhase::Active,
            CrisisPhase::Active => CrisisPhase::Resolving,
            CrisisPhase::Resolving | CrisisPhase::Resolved => CrisisPhase::Resolved,
        };
    }

    /// Establece la fase
    pub fn set_phase(&mut self, phase: CrisisPhase) {
        self.current_phase = phase;
    }

    /// Obtiene la duración total
    pub fn duration(&self) -> u32 {
        self.duration
    }

    /// Establece la duración total
    pub fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
        self.remaining_jornadas = duration;
        self.is_permanent = duration == 0;
    }

    /// Obtiene las jornadas restantes
    pub fn remaining_jornadas(&self) -> u32 {
        self.remaining_jornadas
    }

    /// Disminuye las jornadas restantes
    pub fn decrease_remaining(&mut self) {
        if !self.is_permanent {
            self.remaining_jornadas = self.remaining_jornadas.saturating_sub(1);
        }
    }

    /// Verifica si la crisis es permanente
    pub fn is_permanent(&self) -> bool {
        self.is_permanent
    }

    /// Establece si la crisis es permanente
    pub fn set_permanent(&mut self, is_permanent: bool) {
        self.is_permanent = is_permanent;
        if is_permanent {
            self.remaining_jornadas = 0;
        }
    }

    /// Verifica si la crisis ha terminado
    pub fn is_resolved(&self) -> bool {
        matches!(self.current_phase, CrisisPhase::Resolved) ||
        (!self.is_permanent && self.remaining_jornadas == 0)
    }

    /// Obtiene un efecto específico
    pub fn get_effect(&self, property: &str) -> Option<&i32> {
        self.effects.get(property)
    }

    /// Obtiene todos los efectos
    pub fn effects(&self) -> &HashMap<String, i32> {
        &self.effects
    }

    /// Establece un efecto
    pub fn set_effect(&mut self, property: String, value: i32) {
        self.effects.insert(property, value);
    }

    /// Elimina un efecto
    pub fn remove_effect(&mut self, property: &str) {
        self.effects.remove(property);
    }

    /// Limpia todos los efectos
    pub fn clear_effects(&mut self) {
        self.effects.clear();
    }
}

impl Default for Crisis {
    fn default() -> Self {
        Self::new(
            CrisisId::from("crisis_default"),
            CrisisSeverity::Medium,
            "Crisis genérica".to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crisis_creation() {
        let crisis = Crisis::new(
            CrisisId::from("crisis_1"),
            CrisisSeverity::High,
            "Descripción de prueba".to_string()
        );
        assert_eq!(crisis.id().as_str(), "crisis_1");
        assert!(matches!(crisis.severity(), CrisisSeverity::High));
        assert!(matches!(crisis.current_phase(), CrisisPhase::Pending));
    }

    #[test]
    fn test_crisis_phases() {
        let mut crisis = Crisis::new(CrisisId::from("c1"), CrisisSeverity::Low, "".to_string());
        assert!(matches!(crisis.current_phase(), CrisisPhase::Pending));
        crisis.advance_phase();
        assert!(matches!(crisis.current_phase(), CrisisPhase::Active));
        crisis.advance_phase();
        assert!(matches!(crisis.current_phase(), CrisisPhase::Resolving));
        crisis.advance_phase();
        assert!(matches!(crisis.current_phase(), CrisisPhase::Resolved));
    }

    #[test]
    fn test_crisis_duration() {
        let mut crisis = Crisis::new(CrisisId::from("c1"), CrisisSeverity::Medium, "".to_string());
        crisis.set_duration(5);
        assert_eq!(crisis.duration(), 5);
        assert_eq!(crisis.remaining_jornadas(), 5);
        crisis.decrease_remaining();
        assert_eq!(crisis.remaining_jornadas(), 4);
    }

    #[test]
    fn test_permanent_crisis() {
        let mut crisis = Crisis::new(CrisisId::from("c1"), CrisisSeverity::High, "".to_string());
        crisis.set_permanent(true);
        assert!(crisis.is_permanent());
        crisis.decrease_remaining();
        assert_eq!(crisis.remaining_jornadas(), 0);
    }
}
