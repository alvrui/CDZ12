use crate::error::ProtagonistError;
use crate::models::{Medidores, Tendencia};

impl Medidores {
    pub const MIN_VALUE: u8 = 0;
    pub const MAX_VALUE: u8 = 100;

    pub fn increase_fuerza(&mut self, value: i8) -> Result<(), ProtagonistError> {
        let new_val = (self.fuerza as i16) + (value as i16);
        if new_val < Self::MIN_VALUE as i16 || new_val > Self::MAX_VALUE as i16 {
            return Err(ProtagonistError::AttributeOutOfBounds(
                "fuerza".to_string(),
                Self::MIN_VALUE,
                Self::MAX_VALUE,
            ));
        }
        self.fuerza = new_val as u8;
        Ok(())
    }

    pub fn decrease_fuerza(&mut self, value: i8) -> Result<(), ProtagonistError> {
        self.increase_fuerza(-value)
    }

    pub fn increase_influencia(&mut self, value: i8) -> Result<(), ProtagonistError> {
        let new_val = (self.influencia as i16) + (value as i16);
        if new_val < Self::MIN_VALUE as i16 || new_val > Self::MAX_VALUE as i16 {
            return Err(ProtagonistError::AttributeOutOfBounds(
                "influencia".to_string(),
                Self::MIN_VALUE,
                Self::MAX_VALUE,
            ));
        }
        self.influencia = new_val as u8;
        Ok(())
    }

    pub fn decrease_influencia(&mut self, value: i8) -> Result<(), ProtagonistError> {
        self.increase_influencia(-value)
    }

    pub fn increase_recursos(&mut self, value: i8) -> Result<(), ProtagonistError> {
        let new_val = (self.recursos as i16) + (value as i16);
        if new_val < Self::MIN_VALUE as i16 || new_val > Self::MAX_VALUE as i16 {
            return Err(ProtagonistError::AttributeOutOfBounds(
                "recursos".to_string(),
                Self::MIN_VALUE,
                Self::MAX_VALUE,
            ));
        }
        self.recursos = new_val as u8;
        Ok(())
    }

    pub fn decrease_recursos(&mut self, value: i8) -> Result<(), ProtagonistError> {
        self.increase_recursos(-value)
    }

    pub fn increase_reputacion(&mut self, value: i8) -> Result<(), ProtagonistError> {
        let new_val = (self.reputacion as i16) + (value as i16);
        if new_val < Self::MIN_VALUE as i16 || new_val > Self::MAX_VALUE as i16 {
            return Err(ProtagonistError::AttributeOutOfBounds(
                "reputacion".to_string(),
                Self::MIN_VALUE,
                Self::MAX_VALUE,
            ));
        }
        self.reputacion = new_val as u8;
        Ok(())
    }

    pub fn decrease_reputacion(&mut self, value: i8) -> Result<(), ProtagonistError> {
        self.increase_reputacion(-value)
    }

    pub fn increase_lealtad(&mut self, value: i8) -> Result<(), ProtagonistError> {
        let new_val = (self.lealtad as i16) + (value as i16);
        if new_val < Self::MIN_VALUE as i16 || new_val > Self::MAX_VALUE as i16 {
            return Err(ProtagonistError::AttributeOutOfBounds(
                "lealtad".to_string(),
                Self::MIN_VALUE,
                Self::MAX_VALUE,
            ));
        }
        self.lealtad = new_val as u8;
        Ok(())
    }

    pub fn decrease_lealtad(&mut self, value: i8) -> Result<(), ProtagonistError> {
        self.increase_lealtad(-value)
    }

    pub fn increase_salud(&mut self, value: i8) -> Result<(), ProtagonistError> {
        let new_val = (self.salud as i16) + (value as i16);
        if new_val < Self::MIN_VALUE as i16 || new_val > Self::MAX_VALUE as i16 {
            return Err(ProtagonistError::AttributeOutOfBounds(
                "salud".to_string(),
                Self::MIN_VALUE,
                Self::MAX_VALUE,
            ));
        }
        self.salud = new_val as u8;
        Ok(())
    }

    pub fn decrease_salud(&mut self, value: i8) -> Result<(), ProtagonistError> {
        self.increase_salud(-value)
    }

    pub fn set_tendencia(&mut self, tendencia: Tendencia) {
        self.tendencia = tendencia;
    }

    pub fn get_tendencia(&self) -> &Tendencia {
        &self.tendencia
    }
}