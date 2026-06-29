use crate::error::ProtagonistError;
use crate::models::Protagonist;
use std::collections::HashMap;

pub struct ProtagonistManager {
    protagonists: HashMap<String, Protagonist>,
    current_protagonist_id: Option<String>,
}

impl ProtagonistManager {
    pub fn new() -> Self {
        Self {
            protagonists: HashMap::new(),
            current_protagonist_id: None,
        }
    }

    pub fn add_protagonist(&mut self, protagonist: Protagonist) {
        self.protagonists
            .insert(protagonist.id.id.clone(), protagonist);
    }

    pub fn remove_protagonist(&mut self, protagonist_id: &str) -> Option<Protagonist> {
        if self.current_protagonist_id.as_deref() == Some(protagonist_id) {
            self.current_protagonist_id = None;
        }
        self.protagonists.remove(protagonist_id)
    }

    pub fn set_current_protagonist(&mut self, protagonist_id: &str) -> Result<(), ProtagonistError> {
        if self.protagonists.contains_key(protagonist_id) {
            self.current_protagonist_id = Some(protagonist_id.to_string());
            Ok(())
        } else {
            Err(ProtagonistError::ValidationError(format!(
                "Protagonist {} not found",
                protagonist_id
            )))
        }
    }

    pub fn get_current_protagonist(&self) -> Result<&Protagonist, ProtagonistError> {
        self.current_protagonist_id
            .as_deref()
            .and_then(|id| self.protagonists.get(id))
            .ok_or_else(|| {
                ProtagonistError::ValidationError("No current protagonist set".to_string())
            })
    }

    pub fn get_current_protagonist_mut(&mut self) -> Result<&mut Protagonist, ProtagonistError> {
        self.current_protagonist_id
            .as_deref()
            .and_then(|id| self.protagonists.get_mut(id))
            .ok_or_else(|| {
                ProtagonistError::ValidationError("No current protagonist set".to_string())
            })
    }

    pub fn get_protagonist(&self, protagonist_id: &str) -> Option<&Protagonist> {
        self.protagonists.get(protagonist_id)
    }

    pub fn get_protagonist_mut(&mut self, protagonist_id: &str) -> Option<&mut Protagonist> {
        self.protagonists.get_mut(protagonist_id)
    }
}
