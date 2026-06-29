//! Modulo M1: Estado del Mundo
//!
//! Este modulo implementa la gestion del estado global del mundo de juego,
//! independiente de cualquier contexto historico concreto.
//!
//! # Responsabilidades
//!
//! - Gestion del tiempo abstracto (tramos, actos, jornadas)
//! - Gestion de la polarizacion politica abstracta
//! - Gestion de la visibilidad del tablero
//! - Gestion de facciones genericas
//! - Gestion de espacios genericos
//! - Gestion de crisis
//!
//! # Principios
//!
//! - **NUNCA** contiene referencias a elementos historicos concretos
//! - Todos los IDs son abstractos (numericos o con prefijos genericos)
//! - Toda la logica es independiente del contenido
//! - Separacion estricta entre motor (Nivel 1) y configuracion (Nivel 2)

#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::needless_doctest_main)]

pub mod error;
pub mod entities;
pub mod manager;
pub mod models;
pub mod polarization;
pub mod serialization;
pub mod time;
pub mod visibility;

// Re-exportaciones para conveniencia
pub use error::*;
pub use entities::*;
pub use manager::*;
pub use models::*;
pub use polarization::*;
pub use serialization::*;
pub use time::*;
pub use visibility::*;

/// Version del modulo para compatibilidad de serializacion
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
