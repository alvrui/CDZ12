use crate::error::ProtagonistError;
use crate::models::{Objective, Objectives};

impl Objectives {
    pub fn add_objective(&mut self, objective_id: String, objective: Objective) {
        self.list.insert(objective_id, objective);
    }

    pub fn remove_objective(&mut self, objective_id: &str) -> Result<Objective, ProtagonistError> {
        self.list
            .remove(objective_id)
            .ok_or_else(|| ProtagonistError::ObjectiveNotFound(objective_id.to_string()))
    }

    pub fn get_objective(&self, objective_id: &str) -> Result<&Objective, ProtagonistError> {
        self.list
            .get(objective_id)
            .ok_or_else(|| ProtagonistError::ObjectiveNotFound(objective_id.to_string()))
    }

    pub fn update_progress(
        &mut self,
        objective_id: &str,
        progress: u32,
    ) -> Result<(), ProtagonistError> {
        if let Some(objective) = self.list.get_mut(objective_id) {
            objective.progress = progress;
            Ok(())
        } else {
            Err(ProtagonistError::ObjectiveNotFound(objective_id.to_string()))
        }
    }

    pub fn increase_progress(
        &mut self,
        objective_id: &str,
        amount: u32,
    ) -> Result<(), ProtagonistError> {
        if let Some(objective) = self.list.get_mut(objective_id) {
            objective.progress += amount;
            if objective.progress >= 100 {
                objective.completed = true;
            }
            Ok(())
        } else {
            Err(ProtagonistError::ObjectiveNotFound(objective_id.to_string()))
        }
    }

    pub fn mark_completed(&mut self, objective_id: &str) -> Result<(), ProtagonistError> {
        if let Some(objective) = self.list.get_mut(objective_id) {
            objective.completed = true;
            Ok(())
        } else {
            Err(ProtagonistError::ObjectiveNotFound(objective_id.to_string()))
        }
    }
}
