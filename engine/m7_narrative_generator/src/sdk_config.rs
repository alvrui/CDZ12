use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Story Element individual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryElement {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub tags: Vec<String>,
    pub facciones_afines: Vec<String>,
    pub facciones_enemigas: Vec<String>,
    pub requisitos: Option<HashMap<String, serde_json::Value>>,
    pub peso_base: f32,
    pub modificadores: Option<HashMap<String, HashMap<String, f32>>>,
    pub compatibilidad: Option<HashMap<String, HashMap<String, i32>>>,
}

/// Categoría de Story Elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryElementCategory {
    pub descripcion: String,
    pub elementos: Vec<StoryElement>,
}

/// Catálogo completo de Story Elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryElementsCatalog {
    pub version: String,
    pub descripcion: String,
    pub categorias: HashMap<String, StoryElementCategory>,
}

impl StoryElementsCatalog {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let catalog: Self = serde_json::from_str(&content)?;
        Ok(catalog)
    }

    /// Obtiene elementos de una categoría
    pub fn get_elements(&self, category: &str) -> Option<&Vec<StoryElement>> {
        self.categorias.get(category).map(|c| &c.elementos)
    }

    /// Filtra elementos por requisitos de contexto
    pub fn filter_by_requirements(
        &self,
        category: &str,
        contexto: &HashMap<String, String>,
    ) -> Vec<&StoryElement> {
        let elements = match self.get_elements(category) {
            Some(e) => e,
            None => return Vec::new(),
        };

        elements
            .iter()
            .filter(|e| {
                match &e.requisitos {
                    Some(reqs) => {
                        reqs.iter().all(|(key, value)| {
                            contexto.get(key).map_or(false, |ctx_val| {
                                // Comparación simple para demo
                                match value {
                                    serde_json::Value::String(s) => ctx_val == s,
                                    serde_json::Value::Array(arr) => {
                                        arr.iter().any(|v| v.as_str().map_or(false, |v| v == ctx_val))
                                    }
                                    _ => true,
                                }
                            })
                        })
                    }
                    None => true,
                }
            })
            .collect()
    }

    /// Calcula peso ajustado por modificadores de contexto
    pub fn calculate_weight(
        &self,
        element: &StoryElement,
        contexto: &HashMap<String, String>,
    ) -> f32 {
        let mut weight = element.peso_base;

        if let Some(mods) = &element.modificadores {
            for (mod_type, values) in mods {
                if let Some(ctx_value) = contexto.get(mod_type) {
                    if let Some(modifier) = values.get(ctx_value) {
                        weight *= modifier;
                    }
                }
            }
        }

        weight
    }

    /// Selecciona el mejor elemento de una categoría para el contexto
    pub fn select_best(
        &self,
        category: &str,
        contexto: &HashMap<String, String>,
    ) -> Option<&StoryElement> {
        let filtered = self.filter_by_requirements(category, contexto);
        
        filtered
            .into_iter()
            .max_by(|a, b| {
                let wa = self.calculate_weight(a, contexto);
                let wb = self.calculate_weight(b, contexto);
                wa.partial_cmp(&wb).unwrap_or(std::cmp::Ordering::Equal)
            })
    }
}

/// Configuración de tonos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TonoConfig {
    pub version: String,
    pub parametros: HashMap<String, serde_json::Value>,
    pub tonos_por_acto: HashMap<String, serde_json::Value>,
}

impl TonoConfig {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn get_tono_for_acto(&self, acto: &str) -> Option<&serde_json::Value> {
        self.tonos_por_acto.get(acto)
    }
}

/// Configuración de fases
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FasesConfig {
    pub version: String,
    pub actos: Vec<ActoConfig>,
    pub climas_politicos: HashMap<String, ClimaPoliticoConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActoConfig {
    pub acto: u8,
    pub nombre: String,
    pub descripcion: String,
    pub fecha_inicio: String,
    pub fecha_fin: String,
    pub tramos: Vec<TramoConfig>,
    pub eventos_pivote: Vec<EventoPivoteConfig>,
    pub objetivos: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TramoConfig {
    pub tramo_id: String,
    pub eventos: Vec<String>,
    pub peso: f32,
    pub descripcion: Option<String>,
    pub requiere: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventoPivoteConfig {
    pub evento_id: String,
    pub nombre: String,
    pub fecha: String,
    pub descripcion: String,
    pub impacto: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClimaPoliticoConfig {
    pub peso: f32,
    pub descripcion: String,
    pub eventos_prohibidos: Option<Vec<String>>,
    pub eventos_favorecidos: Option<Vec<String>>,
    pub cooldown_reducido: Option<bool>,
}

impl FasesConfig {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Self = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn get_acto(&self, acto: u8) -> Option<&ActoConfig> {
        self.actos.iter().find(|a| a.acto == acto)
    }
}

/// Configuración completa del SDK
#[derive(Debug, Clone)]
pub struct SdkConfig {
    pub story_elements: StoryElementsCatalog,
    pub tonos: TonoConfig,
    pub fases: FasesConfig,
}

impl SdkConfig {
    pub fn load(config_dir: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            story_elements: StoryElementsCatalog::load(&format!("{}/story_elements.json", config_dir))?,
            tonos: TonoConfig::load(&format!("{}/tonos.json", config_dir))?,
            fases: FasesConfig::load(&format!("{}/fases.json", config_dir))?,
        })
    }
}