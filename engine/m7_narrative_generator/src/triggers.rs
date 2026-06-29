use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::error::NarrativeError;
use crate::models::{ConditionOperator, GameContext, NarrativeTrigger, TriggerCondition, VariableType};

/// Almacén de triggers configurados
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TriggerStore {
    pub triggers: HashMap<String, NarrativeTrigger>,
}

impl TriggerStore {
    pub fn new() -> Self {
        Self {
            triggers: HashMap::new(),
        }
    }

    pub fn add_trigger(&mut self, trigger: NarrativeTrigger) {
        self.triggers.insert(trigger.id.id.clone(), trigger);
    }

    pub fn get_trigger(&self, id: &str) -> Result<&NarrativeTrigger, NarrativeError> {
        self.triggers
            .get(id)
            .ok_or_else(|| NarrativeError::TriggerNotFound(id.to_string()))
    }

    pub fn evaluate_trigger(&self, trigger_id: &str, context: &GameContext) -> Result<bool, NarrativeError> {
        let trigger = self.get_trigger(trigger_id)?;
        for condition in &trigger.conditions {
            if !self.evaluate_condition(condition, context)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub fn evaluate_condition(
        &self,
        condition: &TriggerCondition,
        context: &GameContext,
    ) -> Result<bool, NarrativeError> {
        let value = self.get_variable_value(&condition.variable_name, &condition.variable_type, context)?;

        match condition.operator {
            ConditionOperator::GreaterThan => {
                if let (Ok(val1), Ok(val2)) = (value.parse::<f64>(), condition.value.parse::<f64>()) {
                    Ok(val1 > val2)
                } else {
                    Err(NarrativeError::ConditionEvaluationError(format!(
                        "Cannot compare {} and {} as numbers",
                        value, condition.value
                    )))
                }
            }
            ConditionOperator::LessThan => {
                if let (Ok(val1), Ok(val2)) = (value.parse::<f64>(), condition.value.parse::<f64>()) {
                    Ok(val1 < val2)
                } else {
                    Err(NarrativeError::ConditionEvaluationError(format!(
                        "Cannot compare {} and {} as numbers",
                        value, condition.value
                    )))
                }
            }
            ConditionOperator::Equals => Ok(value == condition.value),
            ConditionOperator::NotEquals => Ok(value != condition.value),
            ConditionOperator::GreaterThanOrEquals => {
                if let (Ok(val1), Ok(val2)) = (value.parse::<f64>(), condition.value.parse::<f64>()) {
                    Ok(val1 >= val2)
                } else {
                    Err(NarrativeError::ConditionEvaluationError(format!(
                        "Cannot compare {} and {} as numbers",
                        value, condition.value
                    )))
                }
            }
            ConditionOperator::LessThanOrEquals => {
                if let (Ok(val1), Ok(val2)) = (value.parse::<f64>(), condition.value.parse::<f64>()) {
                    Ok(val1 <= val2)
                } else {
                    Err(NarrativeError::ConditionEvaluationError(format!(
                        "Cannot compare {} and {} as numbers",
                        value, condition.value
                    )))
                }
            }
        }
    }

    fn get_variable_value(
        &self,
        name: &str,
        var_type: &VariableType,
        context: &GameContext,
    ) -> Result<String, NarrativeError> {
        match var_type {
            VariableType::WorldState => context
                .world_state
                .get(name)
                .cloned()
                .ok_or_else(|| NarrativeError::InvalidCondition(format!("World state variable {} not found", name))),
            VariableType::ProtagonistState => context
                .protagonist_state
                .get(name)
                .cloned()
                .ok_or_else(|| NarrativeError::InvalidCondition(format!("Protagonist state variable {} not found", name))),
            VariableType::Medidor => context
                .medidores
                .get(name)
                .cloned()
                .ok_or_else(|| NarrativeError::InvalidCondition(format!("Medidor {} not found", name))),
            VariableType::Event => {
                if context.active_events.contains(&name.to_string()) {
                    Ok("true".to_string())
                } else {
                    Ok("false".to_string())
                }
            }
            VariableType::Decision => {
                if context.decision_history.contains(&name.to_string()) {
                    Ok("true".to_string())
                } else {
                    Ok("false".to_string())
                }
            }
            VariableType::Time => context
                .current_time
                .get(name)
                .cloned()
                .ok_or_else(|| NarrativeError::InvalidCondition(format!("Time variable {} not found", name))),
        }
    }

    pub fn check_triggers(&self, context: &GameContext) -> Result<Vec<&NarrativeTrigger>, NarrativeError> {
        let mut triggered = Vec::new();
        for trigger in self.triggers.values() {
            if self.evaluate_trigger(&trigger.id.id, context)? {
                triggered.push(trigger);
            }
        }
        Ok(triggered)
    }
}
