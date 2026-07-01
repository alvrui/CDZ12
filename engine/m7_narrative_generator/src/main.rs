use crate::chain_generator::{MotorContext, NarrativeChainGenerator};
use crate::sdk_config::SdkConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = "../../config";
    let config = SdkConfig::load(config_dir)?;
    
    let generator = NarrativeChainGenerator::new(config);
    
    let context_path = format!("{}/SDK/motor2sdk.json", config_dir);
    let context = MotorContext::load(&context_path)?;
    
    let solicitud_id = "test_001";
    let chain = generator.generate(&context, solicitud_id)?;
    
    let output_path = format!("{}/SDK/sdk2motor.json", config_dir);
    chain.save(&output_path)?;
    
    println!("Cadena narrativa generada: {}", chain.cadena_id);
    println!("Compatibilidad total: {:.2}", chain.metadata.compatibilidad_total);
    println!("Eventos generados: {}", chain.cadena.eventos.len());
    println!("Resultado guardado en: {}", output_path);
    
    Ok(())
}