use std::path::Path;
use std::fs;
use crate::error::NarrativeError;
use crate::models::NarrativeConfig;

/// Gestor de archivos de configuración generados por el SDK
pub struct ConfigManager {
    config_dir: String,
}

impl ConfigManager {
    pub fn new(config_dir: String) -> Self {
        Self { config_dir }
    }

    /// Carga una configuración narrativa
    pub fn load_config(&self, trigger_id: &str) -> Result<Option<NarrativeConfig>, NarrativeError> {
        let path = Path::new(&self.config_dir)
            .join("responses")
            .join(format!("{}.json", trigger_id));

        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&path)
            .map_err(|e| NarrativeError::ConfigNotFound(format!("Failed to read config: {}", e)))?;

        let config: NarrativeConfig = serde_json::from_str(&content)
            .map_err(|e| NarrativeError::SerializationError(e.to_string()))?;

        Ok(Some(config))
    }

    /// Lista todas las configuraciones disponibles
    pub fn list_configs(&self) -> Result<Vec<String>, NarrativeError> {
        let path = Path::new(&self.config_dir).join("responses");
        if !path.exists() {
            return Ok(Vec::new());
        }

        let entries = fs::read_dir(&path)
            .map_err(|e| NarrativeError::ConfigNotFound(format!("Failed to read directory: {}", e)))?;

        let mut configs = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().extension().map_or(false, |ext| ext == "json") {
                    if let Some(name) = entry.file_name().to_str() {
                        configs.push(name.to_string());
                    }
                }
            }
        }

        Ok(configs)
    }

    /// Elimina una configuración
    pub fn delete_config(&self, trigger_id: &str) -> Result<(), NarrativeError> {
        let path = Path::new(&self.config_dir)
            .join("responses")
            .join(format!("{}.json", trigger_id));

        if path.exists() {
            fs::remove_file(&path)
                .map_err(|e| NarrativeError::ConfigNotFound(format!("Failed to delete config: {}", e)))?;
        }

        Ok(())
    }
}
