use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum MedidorError {
    #[error("Medidor {0} no encontrado")]
    MedidorNoEncontrado(String),

    #[error("Valor de medidor {0} fuera de límites [{1}, {2}]. Valor actual: {3}")]
    ValorFueraDeLimites(String, i32, i32, i32),

    #[error("Error de serialización: {0}")]
    SerializationError(String),
}
