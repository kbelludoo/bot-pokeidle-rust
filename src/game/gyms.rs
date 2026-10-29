use crate::game::types::{PokeGinasioEntry, PokeGinasioInfo, WSMessage};
use std::collections::HashMap;
use std::time::Instant;

pub const GYM_TYPES: [&str; 18] = [
    "normal", "fogo", "agua", "planta", "eletrico",
    "gelo", "lutador", "venenoso", "terra", "voador",
    "psiquico", "inseto", "pedra", "fantasma", "dragao",
    "sombrio", "aco", "fada",
];

pub struct GymManager {
    last_query: Option<Instant>,
    last_challenge: Option<Instant>,
    current_index: usize,
    pub gyms_status: HashMap<String, PokeGinasioEntry>,
    pub pode_registrar: bool,
    pub nivel_min: u32,
}

impl Default for GymManager {
    fn default() -> Self {
        Self::new()
    }
}

impl GymManager {
    pub fn new() -> Self {
        Self {
            last_query: None,
            last_challenge: None,
            current_index: 0,
            gyms_status: HashMap::new(),
            pode_registrar: false,
            nivel_min: 0,
        }
    }

    pub fn update_from_info(&mut self, info: &PokeGinasioInfo) {
        self.pode_registrar = info.pode_registrar;
        self.nivel_min = info.nivel_min;
        for g in &info.ginasios {
            self.gyms_status.insert(g.tipo.clone(), g.clone());
        }
        self.last_query = Some(Instant::now());
    }

    pub fn should_query_info(&self) -> bool {
        match self.last_query {
            Some(t) => t.elapsed().as_secs() > 600,
            None => true,
        }
    }

    pub fn build_query_info_message() -> WSMessage {
        WSMessage {
            t: "ginasio.info".to_string(),
            ..Default::default()
        }
    }

    pub fn pick_next_gym_to_challenge(&mut self, trainer_level: u32) -> Option<String> {
        if let Some(t) = self.last_challenge {
            if t.elapsed().as_secs() < 45 {
                return None;
            }
        }
        if self.nivel_min > 0 && trainer_level < self.nivel_min {
            return None;
        }

        for i in 0..GYM_TYPES.len() {
            let idx = (self.current_index + i) % GYM_TYPES.len();
            let gym_type = GYM_TYPES[idx];
            let is_leader = self.gyms_status.get(gym_type).map(|e| e.sou_lider).unwrap_or(false);

            if !is_leader {
                self.current_index = (idx + 1) % GYM_TYPES.len();
                self.last_challenge = Some(Instant::now());
                return Some(gym_type.to_string());
            }
        }

        None
    }

    pub fn build_gym_challenge_message(gym_type: &str) -> WSMessage {
        WSMessage {
            t: "ginasio.desafiar".to_string(),
            tipo: Some(gym_type.to_string()),
            ..Default::default()
        }
    }
}
