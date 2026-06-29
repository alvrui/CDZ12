use crate::error::EventError;
use crate::models::{Event, EventPool};
use crate::event::EventContext;
use rand::Rng;

pub struct EventGenerator {
    pool: EventPool,
}

impl EventGenerator {
    pub fn new(pool: EventPool) -> Self {
        Self { pool }
    }

    pub fn generate_event(&self, context: &EventContext) -> Result<Option<&Event>, EventError> {
        let mut candidates: Vec<&Event> = self
            .pool
            .events
            .values()
            .filter(|event| {
                event
                    .evaluate_conditions(context)
                    .unwrap_or(false)
            })
            .collect();

        if candidates.is_empty() {
            return Ok(None);
        }

        // Seleccionar aleatoriamente basado en pesos
        let total_weight: f32 = candidates.iter().map(|e| e.weight).sum();
        let mut rng = rand::thread_rng();
        let random_value = rng.gen_range(0.0..total_weight);

        let mut cumulative_weight = 0.0;
        for event in &candidates {
            cumulative_weight += event.weight;
            if random_value <= cumulative_weight {
                return Ok(Some(*event));
            }
        }

        Ok(candidates.get(0).copied())
    }

    pub fn generate_all_events(&self, context: &EventContext) -> Result<Vec<&Event>, EventError> {
        let events: Vec<&Event> = self
            .pool
            .events
            .values()
            .filter(|event| {
                event
                    .evaluate_conditions(context)
                    .unwrap_or(false)
            })
            .collect();

        Ok(events)
    }
}
