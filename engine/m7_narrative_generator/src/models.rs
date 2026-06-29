use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Identificador de un trigger narrativo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TriggerId {
    pub id: String,
}
impl TriggerId {
    pub fn new(id: String) -> Self { Self { id } }
}

/// Tipo de variable para condiciones (de M1-M6)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VariableType {
    WorldState,
    ProtagonistState,
    Medidor,
    Event,
    Decision,
    Time,
}

/// Condición individual para disparar un trigger
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TriggerCondition {
    pub variable_name: String,
    pub variable_type: VariableType,
    pub operator: ConditionOperator,
    pub value: String,
}
impl TriggerCondition {
    pub fn new(
        variable_name: String,
        variable_type: VariableType,
        operator: ConditionOperator,
        value: String,
    ) -> Self {
        Self {
            variable_name,
            variable_type,
            operator,
            value,
        }
    }
}

/// Operadores para condiciones
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionOperator {
    GreaterThan,
    LessThan,
    Equals,
    NotEquals,
    GreaterThanOrEquals,
    LessThanOrEquals,
}

/// Trigger narrativo: conjunto de condiciones que activa una petición al SDK
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NarrativeTrigger {
    pub id: TriggerId,
    pub name: String,
    pub description: String,
    pub conditions: Vec<TriggerCondition>,
    pub priority: u32,
}
impl NarrativeTrigger {
    pub fn new(
        id: TriggerId,
        name: String,
        description: String,
        conditions: Vec<TriggerCondition>,
        priority: u32,
    ) -> Self {
        Self {
            id,
            name,
            description,
            conditions,
            priority,
        }
    }
    pub fn add_condition(&mut self, condition: TriggerCondition) {
        self.conditions.push(condition);
    }
}

/// Contexto actual del juego para enviar al SDK
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GameContext {
    pub world_state: HashMap<String, String>,
    pub protagonist_state: HashMap<String, String>,
    pub medidores: HashMap<String, String>,
    pub current_time: HashMap<String, String>,
    pub active_events: Vec<String>,
    pub decision_history: Vec<String>,
}
impl GameContext {
    pub fn new() -> Self {
        Self {
            world_state: HashMap::new(),
            protagonist_state: HashMap::new(),
            medidores: HashMap::new(),
            current_time: HashMap::new(),
            active_events: Vec::new(),
            decision_history: Vec::new(),
        }
    }
}

/// Petición al SDK
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NarrativeRequest {
    pub trigger_id: TriggerId,
    pub context: GameContext,
    pub timestamp: u64,
}
impl NarrativeRequest {
    pub fn new(trigger_id: TriggerId, context: GameContext) -> Self {
        Self {
            trigger_id,
            context,
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }
}

/// Contenido generado por el SDK (se lee de archivos JSON)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NarrativeConfig {
    pub request_id: String,
    pub trigger_id: TriggerId,
    pub events: Vec<NarrativeEvent>,
    pub narrative_text: String,
    pub decisions: Vec<NarrativeDecision>,
    pub images: Vec<NarrativeImage>,
    pub generated_at: u64,
}
impl NarrativeConfig {
    pub fn new(
        request_id: String,
        trigger_id: TriggerId,
        events: Vec<NarrativeEvent>,
        narrative_text: String,
        decisions: Vec<NarrativeDecision>,
        images: Vec<NarrativeImage>,
    ) -> Self {
        Self {
            request_id,
            trigger_id,
            events,
            narrative_text,
            decisions,
            images,
            generated_at: chrono::Utc::now().timestamp() as u64,
        }
    }
}

/// Evento narrativo generado por el SDK
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NarrativeEvent {
    pub id: String,
    pub event_type: String,
    pub title: String,
    pub description: String,
    pub parameters: HashMap<String, String>,
}

/// Decisión narrativa generada por el SDK
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NarrativeDecision {
    pub id: String,
    pub text: String,
    pub effects: HashMap<String, String>,
    pub next_event: Option<String>,
}

/// Imagen generada por el SDK para UI
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NarrativeImage {
    pub id: String,
    pub path: String,
    pub description: String,
    pub tags: Vec<String>,
}
