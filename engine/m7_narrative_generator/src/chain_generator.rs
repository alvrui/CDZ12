use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::sdk_config::SdkConfig;

/// Contexto del motor (simplificado para demo)
#[derive(Debug, Clone, Deserialize)]
pub struct MotorContext {
    pub world_state: HashMap<String, serde_json::Value>,
    pub protagonist_state: HashMap<String, serde_json::Value>,
}

impl MotorContext {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let context: Self = serde_json::from_str(&content)?;
        Ok(context)
    }

    /// Extrae valores clave para selección de Story Elements
    pub fn extract_selection_context(&self) -> HashMap<String, String> {
        let mut ctx = HashMap::new();

        // Extraer de world_state
        if let Some(ws) = self.world_state.get("time") {
            if let Some(acto) = ws.get("acto") {
                if let Some(acto_str) = acto.as_str() {
                    ctx.insert("acto".to_string(), acto_str.to_string());
                }
            }
        }

        if let Some(ws) = self.world_state.get("clima_politico") {
            if let Some(clima) = ws.as_str() {
                ctx.insert("clima_politico".to_string(), clima.to_string());
            }
        }

        if let Some(ws) = self.world_state.get("etiquetas_mundo") {
            if let Some(tags) = ws.as_array() {
                for tag in tags {
                    if let Some(tag_str) = tag.as_str() {
                        ctx.insert(format!("etiqueta_{}", tag_str), "true".to_string());
                    }
                }
            }
        }

        // Extraer de protagonist_state
        if let Some(ps) = self.protagonist_state.get("perfil") {
            if let Some(perfil) = ps.as_object() {
                for (key, value) in perfil {
                    if let Some(val_str) = value.as_str() {
                        ctx.insert(format!("perfil_{}", key), val_str.to_string());
                    }
                }
            }
        }

        if let Some(ps) = self.protagonist_state.get("etiquetas_protagonista") {
            if let Some(tags) = ps.as_array() {
                for tag in tags {
                    if let Some(tag_str) = tag.as_str() {
                        ctx.insert(format!("etiqueta_prot_{}", tag_str), "true".to_string());
                    }
                }
            }
        }

        ctx
    }
}

/// Cadena narrativa generada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeChain {
    pub version: String,
    pub solicitud_id: String,
    pub cadena_id: String,
    pub timestamp: String,
    pub metadata: ChainMetadata,
    pub cadena: ChainData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainMetadata {
    pub story_elements_utilizados: Vec<UsedElement>,
    pub compatibilidad_total: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsedElement {
    pub categoria: String,
    pub id: String,
    pub nombre: String,
    pub peso: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainData {
    pub titulo_id: String,
    pub tono_aplicado: TonoAplicado,
    pub eventos: Vec<ChainEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TonoAplicado {
    pub estilo: String,
    pub ritmo: String,
    pub violencia: u8,
    pub realismo: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainEvent {
    pub evento_id: String,
    pub familia: String,
    pub titulo_id: String,
    pub descripcion_id: String,
    pub opciones: Vec<EventOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventOption {
    pub id: String,
    pub texto_id: String,
    pub condicion: Option<Condition>,
    pub consecuencias: Vec<Consequence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub medidor: Option<String>,
    pub operador: Option<String>,
    pub valor: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consequence {
    pub target: String,
    pub delta: Option<i32>,
    pub target_id: Option<String>,
    pub tipo: String,
}

/// Generador de cadenas narrativas
pub struct NarrativeChainGenerator {
    config: SdkConfig,
}

impl NarrativeChainGenerator {
    pub fn new(config: SdkConfig) -> Self {
        Self { config }
    }

    /// Genera una cadena narrativa a partir del contexto del motor
    pub fn generate(
        &self,
        context: &MotorContext,
        solicitud_id: &str,
    ) -> Result<NarrativeChain, Box<dyn std::error::Error>> {
        let selection_ctx = context.extract_selection_context();
        
        // Seleccionar un elemento de cada categoría
        let categories = ["protagonista", "antagonista", "personajes_secundarios", "temas", "finales"];
        let mut used_elements = Vec::new();
        let mut total_compatibilidad = 0.0;
        
        for category in categories.iter() {
            if let Some(element) = self.config.story_elements.select_best(category, &selection_ctx) {
                let weight = self.config.story_elements.calculate_weight(element, &selection_ctx);
                used_elements.push(UsedElement {
                    categoria: category.to_string(),
                    id: element.id.clone(),
                    nombre: element.nombre.clone(),
                    peso: weight,
                });
                total_compatibilidad += weight;
            }
        }

        // Calcular compatibilidad promedio
        let avg_compatibilidad = if !used_elements.is_empty() {
            total_compatibilidad / used_elements.len() as f32
        } else {
            0.0
        };

        // Determinar tono basado en acto
        let acto = selection_ctx.get("acto").cloned().unwrap_or_else(|| "Prologo".to_string());
        let tono_key = format!("acto_{}", acto.to_lowercase());
        
        let tono_config = self.config.tonos.get_tono_for_acto(&tono_key)
            .or_else(|| self.config.tonos.get_tono_for_acto("acto_2"))
            .and_then(|t| t.as_object())
            .cloned();

        let tono_aplicado = if let Some(tono) = tono_config {
            TonoAplicado {
                estilo: tono.get("estilo").and_then(|e| e.as_str()).unwrap_or("dramatico").to_string(),
                ritmo: tono.get("ritmo").and_then(|r| r.as_str()).unwrap_or("equilibrado").to_string(),
                violencia: tono.get("violencia_min").and_then(|v| v.as_u64()).map(|v| v as u8).unwrap_or(3),
                realismo: tono.get("realismo_min").and_then(|r| r.as_u64()).map(|r| r as u8).unwrap_or(3),
            }
        } else {
            TonoAplicado {
                estilo: "dramatico".to_string(),
                ritmo: "equilibrado".to_string(),
                violencia: 3,
                realismo: 3,
            }
        };

        // Generar eventos de la cadena
        let mut eventos = Vec::new();
        
        // Para cada Story Element seleccionado, crear un evento
        for element in &used_elements {
            eventos.push(ChainEvent {
                evento_id: format!("evt_{}", element.id),
                familia: "A".to_string(),
                titulo_id: format!("{}_titulo", element.id),
                descripcion_id: format!("{}_descripcion", element.id),
                opciones: self.generate_options(&element.categoria, &selection_ctx),
            });
        }

        Ok(NarrativeChain {
            version: "1.0".to_string(),
            solicitud_id: solicitud_id.to_string(),
            cadena_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            metadata: ChainMetadata {
                story_elements_utilizados: used_elements,
                compatibilidad_total: avg_compatibilidad,
            },
            cadena: ChainData {
                titulo_id: "cadena_generada_titulo".to_string(),
                tono_aplicado,
                eventos,
            },
        })
    }

    fn generate_options(&self, category: &str, contexto: &HashMap<String, String>) -> Vec<EventOption> {
        // Generar opciones basadas en el contexto
        let mut opciones = Vec::new();
        
        // Opción por defecto
        opciones.push(EventOption {
            id: "opc_aceptar".to_string(),
            texto_id: "aceptar_propuesta".to_string(),
            condicion: None,
            consecuencias: vec![Consequence {
                target: "reputacion".to_string(),
                delta: Some(5),
                target_id: None,
                tipo: "inmediata".to_string(),
            }],
        });

        opciones.push(EventOption {
            id: "opc_rechazar".to_string(),
            texto_id: "rechazar_propuesta".to_string(),
            condicion: Some(Condition {
                medidor: Some("influencia".to_string()),
                operador: Some(">=".to_string()),
                valor: Some(50),
            }),
            consecuencias: vec![Consequence {
                target: "influencia".to_string(),
                delta: Some(-5),
                target_id: None,
                tipo: "inmediata".to_string(),
            }],
        });

        opciones
    }
}

impl NarrativeChain {
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}