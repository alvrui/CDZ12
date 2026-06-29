//! Entidad Facción (genérica)
//!
//! Este módulo implementa la estructura de facción de manera completamente
//! abstracta, sin referencias a facciones históricas concretas.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ID de facción (wrapper para asegurar tipo fuerte)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FactionId(pub String);

impl FactionId {
    /// Crea un nuevo ID de facción
    pub fn from(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Obtiene el valor del ID
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for FactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Valor de relación entre facciones (-100 a +100)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RelationValue(pub i8);

impl RelationValue {
    /// Valor mínimo
    pub const MIN: i8 = -100;
    /// Valor máximo
    pub const MAX: i8 = 100;
    /// Valor neutral
    pub const NEUTRAL: i8 = 0;

    /// Crea un nuevo valor de relación
    pub fn new(value: i8) -> Self {
        Self(value.clamp(Self::MIN, Self::MAX))
    }

    /// Obtiene el valor
    pub fn value(&self) -> i8 {
        self.0
    }

    /// Verifica si es positivo
    pub fn is_positive(&self) -> bool {
        self.0 > 0
    }

    /// Verifica si es negativo
    pub fn is_negative(&self) -> bool {
        self.0 < 0
    }

    /// Verifica si es neutral
    pub fn is_neutral(&self) -> bool {
        self.0 == 0
    }
}

impl Default for RelationValue {
    fn default() -> Self {
        Self(0)
    }
}

/// Facción genérica
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Faction {
    /// ID único de la facción
    id: FactionId,
    /// Fuerza de la facción (0-100)
    strength: u8,
    /// Relaciones con otras facciones
    relations: HashMap<FactionId, RelationValue>,
}

impl Faction {
    /// Crea una nueva facción con el ID especificado
    pub fn new(id: FactionId) -> Self {
        Self {
            id,
            strength: 50,
            relations: HashMap::new(),
        }
    }

    /// Obtiene el ID de la facción
    pub fn id(&self) -> &FactionId {
        &self.id
    }

    /// Obtiene la fuerza de la facción
    pub fn strength(&self) -> u8 {
        self.strength
    }

    /// Establece la fuerza de la facción (0-100)
    pub fn set_strength(&mut self, strength: u8) {
        self.strength = strength.clamp(0, 100);
    }

    /// Obtiene la relación con otra facción
    pub fn get_relation(&self, other_id: &FactionId) -> Option<RelationValue> {
        self.relations.get(other_id).copied()
    }

    /// Obtiene la relación con otra facción, o neutral si no existe
    pub fn get_relation_or_default(&self, other_id: &FactionId) -> RelationValue {
        self.get_relation(other_id).unwrap_or_default()
    }

    /// Establece la relación con otra facción
    pub fn set_relation(&mut self, other_id: FactionId, value: RelationValue) {
        self.relations.insert(other_id, value);
    }

    /// Obtiene todas las relaciones
    pub fn relations(&self) -> &HashMap<FactionId, RelationValue> {
        &self.relations
    }

    /// Verifica si tiene relación con otra facción
    pub fn has_relation(&self, other_id: &FactionId) -> bool {
        self.relations.contains_key(other_id)
    }

    /// Elimina la relación con otra facción
    pub fn remove_relation(&mut self, other_id: &FactionId) {
        self.relations.remove(other_id);
    }

    /// Limpia todas las relaciones
    pub fn clear_relations(&mut self) {
        self.relations.clear();
    }

    /// Verifica si es aliada de otra facción (relación > 50)
    pub fn is_allied(&self, other_id: &FactionId) -> bool {
        self.get_relation(other_id)
            .map_or(false, |r| r.value() > 50)
    }

    /// Verifica si es enemiga de otra facción (relación < -50)
    pub fn is_enemy(&self, other_id: &FactionId) -> bool {
        self.get_relation(other_id)
            .map_or(false, |r| r.value() < -50)
    }

    /// Verifica si es neutral con otra facción (|relación| <= 50)
    pub fn is_neutral(&self, other_id: &FactionId) -> bool {
        self.get_relation(other_id)
            .map_or(true, |r| r.value().abs() <= 50)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_faction_creation() {
        let faction = Faction::new(FactionId::from("faccion_1"));
        assert_eq!(faction.id().as_str(), "faccion_1");
        assert_eq!(faction.strength(), 50);
    }

    #[test]
    fn test_faction_strength_bounds() {
        let mut faction = Faction::new(FactionId::from("f1"));
        faction.set_strength(75);
        assert_eq!(faction.strength(), 75);

        faction.set_strength(200);
        assert_eq!(faction.strength(), 100);

        faction.set_strength(0);
        assert_eq!(faction.strength(), 0);
    }

    #[test]
    fn test_faction_relations() {
        let mut faction1 = Faction::new(FactionId::from("f1"));
        let faction2_id = FactionId::from("f2");

        faction1.set_relation(faction2_id.clone(), RelationValue::new(60));
        assert_eq!(
            faction1.get_relation(&faction2_id).unwrap().value(),
            60
        );
        assert!(faction1.is_allied(&faction2_id));

        faction1.set_relation(faction2_id.clone(), RelationValue::new(-60));
        assert!(faction1.is_enemy(&faction2_id));
    }
}
