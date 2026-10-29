use crate::game::types::{
    PokeBicicletaState, PokeBossEntry, PokeCasaState, PokeChocadeiraState, PokeItemPokemon,
    PokeMisticoState, PokeXpShareState, WSMessage,
};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Instant;

pub const ITEM_MYSTIC_TICKET: i32 = 70014;
pub const ITEM_SHINY_EGG_FRAG: i32 = 70012;
pub const ITEM_BICYCLE_FRAG: i32 = 70013;

pub const BEST_BALLS_ORDER: [i32; 5] = [5, 4, 3, 2, 1];
pub const EGG_PRIORITY: [&str; 4] = ["shiny", "p4", "p3", "p2"];

// --- 1. Masmorra Mística (Mystic Dungeon) ---
pub fn check_mystic_entry(
    items: &HashMap<String, i32>,
    mistico: Option<&PokeMisticoState>,
) -> Option<WSMessage> {
    if let Some(m) = mistico {
        if m.species_id > 0 {
            return None;
        }
    }

    let has_ticket = items
        .get(&ITEM_MYSTIC_TICKET.to_string())
        .copied()
        .unwrap_or(0)
        > 0
        || items.get("mysticTicket").copied().unwrap_or(0) > 0;

    if has_ticket {
        Some(WSMessage {
            t: "mistico.entrar".to_string(),
            ..Default::default()
        })
    } else {
        None
    }
}

pub fn pick_best_ball_for_mystic(balls: &HashMap<String, i32>) -> i32 {
    for &ball_id in &BEST_BALLS_ORDER {
        if balls.get(&ball_id.to_string()).copied().unwrap_or(0) > 0 {
            return ball_id;
        }
    }
    1
}

pub fn build_mystic_ball_message(ball_id: i32) -> WSMessage {
    WSMessage {
        t: "mistico.bola".to_string(),
        ball_id: Some(ball_id),
        ..Default::default()
    }
}

// --- 2. Chocadeira (Egg Incubator) ---
pub fn check_incubator_actions(
    choc: Option<&PokeChocadeiraState>,
    items: &HashMap<String, i32>,
) -> Option<WSMessage> {
    // 1. Fabricar Ovo Shiny se tiver 10+ fragmentos
    if items
        .get(&ITEM_SHINY_EGG_FRAG.to_string())
        .copied()
        .unwrap_or(0)
        >= 10
    {
        return Some(WSMessage {
            t: "ovoShiny.fabricar".to_string(),
            ..Default::default()
        });
    }

    let choc = choc?;
    if choc.total <= 0 {
        return None;
    }

    // Achar slots livres
    let mut occupied = std::collections::HashSet::new();
    for o in &choc.ovos {
        occupied.insert(o.slot);
    }

    let mut free_slot = -1;
    for s in 0..choc.total {
        if !occupied.contains(&s) {
            free_slot = s;
            break;
        }
    }

    if free_slot == -1 {
        return None;
    }

    for &egg in &EGG_PRIORITY {
        let has_egg = items.get(egg).copied().unwrap_or(0) > 0
            || items.get(&format!("ovo-{}", egg)).copied().unwrap_or(0) > 0;
        if has_egg {
            return Some(WSMessage {
                t: "chocadeira.colocar".to_string(),
                slot: Some(free_slot),
                ovo: Some(egg.to_string()),
                ..Default::default()
            });
        }
    }

    None
}

// --- 3. Bicicleta (Bicycle) ---
pub fn rarity_score(rar: &str) -> i32 {
    match rar.to_lowercase().as_str() {
        "comum" => 1,
        "incomum" => 2,
        "rara" => 3,
        "mitica" => 4,
        "lendaria" => 5,
        _ => 0,
    }
}

pub fn check_bicycle_actions(
    bici: Option<&PokeBicicletaState>,
    items: &HashMap<String, i32>,
) -> Option<WSMessage> {
    if items
        .get(&ITEM_BICYCLE_FRAG.to_string())
        .copied()
        .unwrap_or(0)
        >= 10
    {
        return Some(WSMessage {
            t: "bicicleta.fabricar".to_string(),
            ..Default::default()
        });
    }

    let bici = bici?;
    if bici.lista.is_empty() {
        return None;
    }

    // Fusão de 5 bicicletas de mesma raridade
    let mut by_rarity: HashMap<String, Vec<Value>> = HashMap::new();
    for b in &bici.lista {
        by_rarity
            .entry(b.raridade.clone())
            .or_default()
            .push(b.id.clone());
    }

    for (rar, ids) in &by_rarity {
        if rar != "lendaria" && ids.len() >= 5 {
            let five = ids.iter().take(5).cloned().collect();
            return Some(WSMessage {
                t: "bicicleta.craftar".to_string(),
                bicicleta_ids: Some(five),
                ..Default::default()
            });
        }
    }

    // Equipar melhor
    let mut best_id: Option<Value> = None;
    let mut best_score = -1;
    for b in &bici.lista {
        let score = rarity_score(&b.raridade);
        if score > best_score {
            best_score = score;
            best_id = Some(b.id.clone());
        }
    }

    if let Some(id) = best_id {
        if bici.equipada.as_ref() != Some(&id) {
            return Some(WSMessage {
                t: "bicicleta.equipar".to_string(),
                bicicleta_id: Some(id),
                ..Default::default()
            });
        }
    }

    None
}

// --- 4. XP Share na Casa (House Passive Training) ---
pub fn check_house_xp_share_actions(
    casa: Option<&PokeCasaState>,
    xp_share: Option<&PokeXpShareState>,
    pokemons: &[PokeItemPokemon],
    active_id: i64,
) -> Option<WSMessage> {
    let xp_share = xp_share?;
    if xp_share.slots.is_empty() {
        return None;
    }

    let casa_id = xp_share
        .casa_id
        .clone()
        .or_else(|| casa.and_then(|c| c.dentro.clone()))?;

    let in_training: std::collections::HashSet<i64> =
        xp_share.slots.iter().map(|s| s.id).collect();

    for (idx, slot) in xp_share.slots.iter().enumerate() {
        if slot.id == 0 {
            for p in pokemons {
                if p.id != active_id && p.slot.is_none() && !in_training.contains(&p.id) {
                    return Some(WSMessage {
                        t: "xpshare.escolher".to_string(),
                        casa_id: Some(casa_id),
                        slot: Some(idx as i32),
                        pokemon_id: Some(p.id),
                        ..Default::default()
                    });
                }
            }
        }
    }

    None
}

// --- 5. Oferenda (Surplus Depot Sacrifice) ---
pub struct OfferingManager {
    last_offering: Option<Instant>,
}

impl Default for OfferingManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OfferingManager {
    pub fn new() -> Self {
        Self {
            last_offering: None,
        }
    }

    pub fn should_run_offering(&mut self, total_pokemons: usize) -> bool {
        if total_pokemons < 10 {
            return false;
        }
        let now = Instant::now();
        if let Some(t) = self.last_offering {
            if now.duration_since(t).as_secs() < 420 {
                return false;
            }
        }
        self.last_offering = Some(now);
        true
    }

    pub fn build_offering_message() -> WSMessage {
        WSMessage {
            t: "oferenda.tudo".to_string(),
            ..Default::default()
        }
    }
}

// --- 6. Boss Rush (Auto-Boss) ---
pub fn default_playable_bosses() -> Vec<PokeBossEntry> {
    vec![
        PokeBossEntry {
            key: "brock".to_string(),
            name: "Brock".to_string(),
            level: 15,
            min_nivel_treinador: 10,
            type_1: "pedra".to_string(),
            type_2: Some("terra".to_string()),
        },
        PokeBossEntry {
            key: "misty".to_string(),
            name: "Misty".to_string(),
            level: 25,
            min_nivel_treinador: 20,
            type_1: "agua".to_string(),
            type_2: None,
        },
        PokeBossEntry {
            key: "surge".to_string(),
            name: "Lt. Surge".to_string(),
            level: 35,
            min_nivel_treinador: 30,
            type_1: "eletrico".to_string(),
            type_2: None,
        },
        PokeBossEntry {
            key: "erika".to_string(),
            name: "Erika".to_string(),
            level: 45,
            min_nivel_treinador: 40,
            type_1: "planta".to_string(),
            type_2: Some("venenoso".to_string()),
        },
        PokeBossEntry {
            key: "giovanni".to_string(),
            name: "Giovanni".to_string(),
            level: 85,
            min_nivel_treinador: 80,
            type_1: "terra".to_string(),
            type_2: None,
        },
    ]
}

pub fn pick_best_boss(
    playable: &[PokeBossEntry],
    trainer_level: u32,
    victories: &HashMap<String, i32>,
) -> Option<PokeBossEntry> {
    let mut eligible: Vec<PokeBossEntry> = playable
        .iter()
        .filter(|b| {
            let req = if b.min_nivel_treinador == 0 {
                b.level
            } else {
                b.min_nivel_treinador
            };
            trainer_level >= req || req <= 10
        })
        .cloned()
        .collect();

    if eligible.is_empty() {
        return playable.first().cloned();
    }

    eligible.sort_by(|a, b| {
        let req_a = if a.min_nivel_treinador == 0 {
            a.level
        } else {
            a.min_nivel_treinador
        };
        let req_b = if b.min_nivel_treinador == 0 {
            b.level
        } else {
            b.min_nivel_treinador
        };
        req_b.cmp(&req_a).then_with(|| {
            let v_a = victories.get(&a.key).copied().unwrap_or(0);
            let v_b = victories.get(&b.key).copied().unwrap_or(0);
            v_a.cmp(&v_b)
        })
    });

    eligible.first().cloned()
}

pub fn build_boss_auto_message(boss_key: &str, active: bool) -> WSMessage {
    WSMessage {
        t: "boss.auto".to_string(),
        ativo: Some(active),
        key: Some(boss_key.to_string()),
        ..Default::default()
    }
}
