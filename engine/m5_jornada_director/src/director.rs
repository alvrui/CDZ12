use crate::error::TiempoError;
use crate::models::{ActoId, Jornada, JornadaId, TemporalState, TramoId};
use crate::time::Tiempo;
use m1_world_state::models::WorldState;
use m2_protagonist_state::models::Protagonist;
use m4_event_generator::error::EventError;
use m4_event_generator::event::EventContext;
use m4_event_generator::mechanics::MechanicsEngine;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use std::collections::HashMap;
use std::path::Path;

pub struct JornadaDirector {
    tiempo: Tiempo,
    state: TemporalState,
    mechanics_engine: MechanicsEngine,
}

impl JornadaDirector {
    pub fn new(tiempo: Tiempo, initial_state: TemporalState) -> Self {
        Self {
            tiempo,
            state: initial_state,
            mechanics_engine: MechanicsEngine::new(),
        }
    }

    pub fn load_mechanics<P: AsRef<Path>>(&mut self, path: P) -> Result<(), EventError> {
        self.mechanics_engine.load_from_file(path)
    }

    pub fn get_state(&self) -> &TemporalState {
        &self.state
    }

    pub fn advance_jornada(
        &mut self,
        world_state: &mut m1_world_state::models::WorldState,
        protagonist_state: &mut m2_protagonist_state::models::Protagonist,
    ) -> Result<Vec<String>, TiempoError> {
        // 1. Avanzar la jornada
        let all_jornadas: Vec<&Jornada> = self.tiempo.jornadas.values().collect();
        let current_jornada_index = all_jornadas
            .iter()
            .position(|j| j.id.id == self.state.jornada_actual.id)
            .ok_or(TiempoError::JornadaNoEncontrada(
                self.state.jornada_actual.id.clone(),
            ))?;

        if current_jornada_index + 1 >= all_jornadas.len() {
            return Err(TiempoError::NoMasJornadas);
        }

        self.state.jornada_actual = all_jornadas[current_jornada_index + 1].id.clone();
        let first_acto = all_jornadas[current_jornada_index + 1]
            .actos
            .first()
            .ok_or(TiempoError::NoMasActosEnJornada)?
            .clone();
        self.state.acto_actual = first_acto;
        let first_tramo = self
            .tiempo
            .get_acto(&self.state.acto_actual.id)?
            .tramos
            .first()
            .ok_or(TiempoError::NoMasTramosEnActo)?
            .clone();
        self.state.tramo_actual = first_tramo;
        self.state.tiempo_transcurrido = 0;

        // 2. Crear contexto y ejecutar mecánicas
        let mut context = self.build_context(world_state, protagonist_state)?;
        let triggered = self.mechanics_engine.evaluate_and_apply(&mut context)?;

        // 3. Aplicar cambios de vuelta a los estados
        self.apply_context_changes(&context, world_state, protagonist_state)?;

        Ok(triggered)
    }

    fn build_context(
        &self,
        world_state: &m1_world_state::models::WorldState,
        protagonist_state: &m2_protagonist_state::models::Protagonist,
    ) -> Result<EventContext, TiempoError> {
        let mut context = EventContext::new();
        let world_value = serde_json::to_value(world_state)?;
        let protagonist_value = serde_json::to_value(protagonist_state)?;

        if let Value::Object(map) = world_value {
            for (k, v) in map {
                context.set_path(&format!("world.{}", k), v)?;
            }
        }
        if let Value::Object(map) = protagonist_value {
            for (k, v) in map {
                context.set_path(&format!("protagonista.{}", k), v)?;
            }
        }
        Ok(context)
    }

    fn apply_context_changes(
        &self,
        context: &EventContext,
        world_state: &mut m1_world_state::models::WorldState,
        protagonist_state: &mut m2_protagonist_state::models::Protagonist,
    ) -> Result<(), TiempoError> {
        if let Value::Object(state_obj) = &context.state {
            if let Some(Value::Object(world_obj)) = state_obj.get("world") {
                *world_state = serde_json::from_value(Value::Object(world_obj.clone()))?;
            }
            if let Some(Value::Object(protagonist_obj)) = state_obj.get("protagonista") {
                *protagonist_state =
                    serde_json::from_value(Value::Object(protagonist_obj.clone()))?;
            }
        }
        Ok(())
    }
    pub fn advance_tramo(&mut self) -> Result<(), TiempoError> {
        let current_acto = self.tiempo.get_acto(&self.state.acto_actual.id)?;
        let current_tramo_index = current_acto
            .tramos
            .iter()
            .position(|t| t.id == self.state.tramo_actual.id)
            .ok_or(TiempoError::TramoNoEncontrado(
                self.state.tramo_actual.id.clone(),
            ))?;

        if current_tramo_index + 1 < current_acto.tramos.len() {
            self.state.tramo_actual = current_acto.tramos[current_tramo_index + 1].clone();
            self.state.tiempo_transcurrido = 0;
            Ok(())
        } else {
            Err(TiempoError::NoMasTramosEnActo)
        }
    }

    pub fn advance_acto(&mut self) -> Result<(), TiempoError> {
        let current_jornada = self.tiempo.get_jornada(&self.state.jornada_actual.id)?;
        let current_acto_index = current_jornada
            .actos
            .iter()
            .position(|a| a.id == self.state.acto_actual.id)
            .ok_or(TiempoError::ActoNoEncontrado(
                self.state.acto_actual.id.clone(),
            ))?;

        if current_acto_index + 1 < current_jornada.actos.len() {
            self.state.acto_actual = current_jornada.actos[current_acto_index + 1].clone();
            let first_tramo = self
                .tiempo
                .get_acto(&self.state.acto_actual.id)?
                .tramos
                .first()
                .ok_or(TiempoError::NoMasTramosEnActo)?
                .clone();
            self.state.tramo_actual = first_tramo;
            self.state.tiempo_transcurrido = 0;
            Ok(())
        } else {
            Err(TiempoError::NoMasActosEnJornada)
        }
    }

    pub fn increment_time(&mut self, amount: u32) -> Result<(), TiempoError> {
        let current_tramo = self.tiempo.get_tramo(&self.state.tramo_actual.id)?;
        if self.state.tiempo_transcurrido + amount > current_tramo.duracion {
            return Err(TiempoError::NoMasTramosEnActo);
        }
        self.state.tiempo_transcurrido += amount;
        Ok(())
    }

    pub fn reset_time(&mut self) {
        self.state.tiempo_transcurrido = 0;
    }
}
