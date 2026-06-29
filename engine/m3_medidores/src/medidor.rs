use crate::error::MedidorError;
use crate::models::{Medidor, MedidorId, TipoMedidor};

impl Medidor {
    pub fn aumentar(&mut self, cantidad: i32) -> Result<(), MedidorError> {
        let nuevo_valor = self.valor + cantidad;
        if nuevo_valor < self.min || nuevo_valor > self.max {
            return Err(MedidorError::ValorFueraDeLimites(
                self.id.id.clone(),
                self.min,
                self.max,
                nuevo_valor,
            ));
        }
        self.valor = nuevo_valor;
        Ok(())
    }

    pub fn disminuir(&mut self, cantidad: i32) -> Result<(), MedidorError> {
        let nuevo_valor = self.valor - cantidad;
        if nuevo_valor < self.min || nuevo_valor > self.max {
            return Err(MedidorError::ValorFueraDeLimites(
                self.id.id.clone(),
                self.min,
                self.max,
                nuevo_valor,
            ));
        }
        self.valor = nuevo_valor;
        Ok(())
    }

    pub fn establecer(&mut self, valor: i32) -> Result<(), MedidorError> {
        if valor < self.min || valor > self.max {
            return Err(MedidorError::ValorFueraDeLimites(
                self.id.id.clone(),
                self.min,
                self.max,
                valor,
            ));
        }
        self.valor = valor;
        Ok(())
    }

    pub fn resetear(&mut self) -> Result<(), MedidorError> {
        self.valor = self.min;
        Ok(())
    }

    pub fn porcentajear(&self) -> f32 {
        if self.max <= self.min {
            0.0
        } else {
            ((self.valor - self.min) as f32 / (self.max - self.min) as f32) * 100.0
        }
    }
}
