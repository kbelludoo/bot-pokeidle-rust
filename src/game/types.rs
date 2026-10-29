use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub enabled: bool,
    pub credentials: GameCredentials,
    #[serde(default)]
    pub settings: GameSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameCredentials {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameSettings {
    #[serde(default)]
    pub hunt_target: String,
    #[serde(default = "default_true")]
    pub auto_hunt: bool,
    #[serde(default = "default_true")]
    pub auto_sell: bool,
    #[serde(default = "default_true")]
    pub auto_heal: bool,
    #[serde(default = "default_true")]
    pub auto_daily: bool,
    #[serde(default = "default_true")]
    pub auto_catch: bool,
    #[serde(default = "default_true")]
    pub auto_boss: bool,
    #[serde(default = "default_true")]
    pub auto_gym: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WSMessage {
    pub t: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispositivo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,

    #[serde(rename = "autoBallSemParar", skip_serializing_if = "Option::is_none")]
    pub auto_ball_sem_parar: Option<bool>,
    #[serde(rename = "autoBallAteCapturar", skip_serializing_if = "Option::is_none")]
    pub auto_ball_ate_capturar: Option<bool>,
    #[serde(rename = "ballIds", skip_serializing_if = "Option::is_none")]
    pub ball_ids: Option<Vec<i32>>,
    #[serde(rename = "autoRevive", skip_serializing_if = "Option::is_none")]
    pub auto_revive: Option<bool>,
    #[serde(rename = "autoPotion", skip_serializing_if = "Option::is_none")]
    pub auto_potion: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ativo: Option<bool>,
    #[serde(rename = "avisoVisto", skip_serializing_if = "Option::is_none")]
    pub aviso_visto: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipo: Option<String>,
    #[serde(rename = "ballId", skip_serializing_if = "Option::is_none")]
    pub ball_id: Option<i32>,
    #[serde(rename = "pokemonId", skip_serializing_if = "Option::is_none")]
    pub pokemon_id: Option<i64>,
    #[serde(rename = "pokemonIds", skip_serializing_if = "Option::is_none")]
    pub pokemon_ids: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ovo: Option<String>,
    #[serde(rename = "bicicletaId", skip_serializing_if = "Option::is_none")]
    pub bicicleta_id: Option<Value>,
    #[serde(rename = "bicicletaIds", skip_serializing_if = "Option::is_none")]
    pub bicicleta_ids: Option<Vec<Value>>,
    #[serde(rename = "casaId", skip_serializing_if = "Option::is_none")]
    pub casa_id: Option<Value>,

    // Inbound fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estado: Option<PokeState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hunts: Option<Vec<PokeHuntArea>>,
    #[serde(rename = "bossesJogaveis", skip_serializing_if = "Option::is_none")]
    pub bosses_jogaveis: Option<Vec<PokeBossEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<PokeGinasioInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desafio: Option<PokeGinasioDesafio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mistico: Option<PokeMisticoState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeState {
    #[serde(default)]
    pub cheio: bool,
    #[serde(default)]
    pub nick: String,
    #[serde(default)]
    pub level: u32,
    #[serde(default)]
    pub xp: i64,
    #[serde(rename = "xpNivel", default)]
    pub xp_nivel: i64,
    #[serde(rename = "xpProximo", default)]
    pub xp_proximo: i64,
    #[serde(default)]
    pub gold: i64,
    #[serde(default)]
    pub diamonds: i64,
    #[serde(default)]
    pub orbs: i64,
    #[serde(rename = "activeId", default)]
    pub active_id: i64,
    #[serde(rename = "huntSlug", default)]
    pub hunt_slug: String,
    #[serde(rename = "noCentro", default)]
    pub no_centro: bool,
    #[serde(default)]
    pub pokemons: Vec<PokeItemPokemon>,
    #[serde(rename = "pkMud", default)]
    pub pk_mud: Vec<PokeItemPokemon>,
    #[serde(rename = "pkFora", default)]
    pub pk_fora: Vec<i64>,
    #[serde(default)]
    pub items: HashMap<String, i32>,
    #[serde(default)]
    pub balls: HashMap<String, i32>,
    #[serde(default)]
    pub boss: Option<PokeBossState>,
    #[serde(default)]
    pub chocadeira: Option<PokeChocadeiraState>,
    #[serde(default)]
    pub bicicleta: Option<PokeBicicletaState>,
    #[serde(default)]
    pub casa: Option<PokeCasaState>,
    #[serde(rename = "xpShare", default)]
    pub xp_share: Option<PokeXpShareState>,
    #[serde(default)]
    pub mistico: Option<PokeMisticoState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeItemPokemon {
    #[serde(default)]
    pub id: i64,
    #[serde(rename = "pokeId", default)]
    pub poke_id: i32,
    #[serde(default, alias = "nome")]
    pub name: String,
    #[serde(default)]
    pub level: u32,
    #[serde(default)]
    pub hp: u32,
    #[serde(rename = "maxHp", default)]
    pub max_hp: u32,
    #[serde(default)]
    pub poder: u32,
    #[serde(default)]
    pub shiny: bool,
    #[serde(default)]
    pub slot: Option<i32>,
    #[serde(default)]
    pub starter: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeHuntArea {
    #[serde(default)]
    pub slug: String,
    #[serde(default, alias = "nome")]
    pub name: String,
    #[serde(default)]
    pub tier: u32,
    #[serde(default)]
    pub pokes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeBossEntry {
    pub key: String,
    #[serde(default, alias = "nome")]
    pub name: String,
    #[serde(default)]
    pub level: u32,
    #[serde(rename = "minNivelTreinador", default)]
    pub min_nivel_treinador: u32,
    #[serde(rename = "type1", default)]
    pub type_1: String,
    #[serde(rename = "type2", default)]
    pub type_2: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeBossState {
    #[serde(default)]
    pub vitorias: HashMap<String, i32>,
    #[serde(default)]
    pub tokens: i32,
    #[serde(default)]
    pub auto: bool,
    #[serde(rename = "autoKey", default)]
    pub auto_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeEggSlot {
    pub slot: i32,
    pub ovo: String,
    #[serde(default)]
    pub inicio: i64,
    #[serde(default)]
    pub fim: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeChocadeiraState {
    #[serde(default)]
    pub total: i32,
    #[serde(default)]
    pub ovos: Vec<PokeEggSlot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeBicicletaItem {
    pub id: Value,
    #[serde(default)]
    pub raridade: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeBicicletaState {
    pub equipada: Option<Value>,
    #[serde(rename = "equipadaRar", default)]
    pub equipada_rar: String,
    #[serde(default)]
    pub lista: Vec<PokeBicicletaItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeCasaState {
    pub dentro: Option<Value>,
    pub lista: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeXpShareSlot {
    #[serde(default)]
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeXpShareState {
    #[serde(rename = "casaId")]
    pub casa_id: Option<Value>,
    #[serde(default)]
    pub slots: Vec<PokeXpShareSlot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeMisticoState {
    #[serde(rename = "speciesId", default)]
    pub species_id: i32,
    #[serde(default)]
    pub nome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeGinasioEntry {
    pub tipo: String,
    #[serde(rename = "souLider", default)]
    pub sou_lider: bool,
    #[serde(rename = "liderNick", default)]
    pub lider_nick: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeGinasioInfo {
    #[serde(rename = "podeRegistrar", default)]
    pub pode_registrar: bool,
    #[serde(rename = "nivelMin", default)]
    pub nivel_min: u32,
    #[serde(default)]
    pub ginasios: Vec<PokeGinasioEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PokeGinasioDesafio {
    pub tipo: String,
    #[serde(default)]
    pub venci: bool,
    #[serde(rename = "tomouLideranca", default)]
    pub tomou_lideranca: bool,
}
