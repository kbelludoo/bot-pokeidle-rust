use crate::engine::circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};
use crate::engine::stamina::{StaminaConfig, StaminaController};
use crate::game::features::{
    build_boss_auto_message, build_mystic_ball_message, check_bicycle_actions,
    check_house_xp_share_actions, check_incubator_actions, check_mystic_entry,
    default_playable_bosses, pick_best_ball_for_mystic, pick_best_boss, OfferingManager,
};
use crate::game::gyms::GymManager;
use crate::game::types::{
    AccountConfig, PokeBossEntry, PokeBossState, PokeItemPokemon, PokeState, WSMessage,
};
use crate::network::socket::PokeSocketClient;
use crate::subsystem::dailies::DailyManager;
use crate::subsystem::inventory::InventoryManager;
use crate::subsystem::potions::PotionManager;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, warn};

pub struct BotInstance {
    pub config: AccountConfig,
    pub circuit_breaker: Arc<CircuitBreaker>,
    pub stamina: Arc<StaminaController>,
    pub inventory: Arc<InventoryManager>,
    pub dailies: Arc<DailyManager>,
    pub potions: Arc<PotionManager>,
    pub socket: Arc<PokeSocketClient>,
    pub gym_manager: Arc<Mutex<GymManager>>,
    pub offering_manager: Arc<Mutex<OfferingManager>>,

    // Live Game State
    pub state: Arc<Mutex<PokeState>>,
    pub pokemons: Arc<Mutex<HashMap<i64, PokeItemPokemon>>>,
    pub playable_bosses: Arc<Mutex<Vec<PokeBossEntry>>>,

    // Metrics
    pub kills: AtomicU64,
    pub gold: AtomicU64,
    pub exp: AtomicU64,
    pub current_level: AtomicU32,
    pub running: AtomicBool,
}

impl BotInstance {
    pub fn new(config: AccountConfig) -> Self {
        let cb_cfg = CircuitBreakerConfig::default();
        let cb = Arc::new(CircuitBreaker::new(&config.id, cb_cfg));
        let stam = Arc::new(StaminaController::new(StaminaConfig::default()));

        let ws_url = if config.credentials.origin.is_empty() {
            "wss://pokeidle.io".to_string()
        } else {
            let orig = &config.credentials.origin;
            let scheme = if orig.starts_with("https") { "wss" } else { "ws" };
            let host = orig.trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/');
            format!("{}://{}", scheme, host)
        };

        let socket = Arc::new(PokeSocketClient::new(&ws_url));

        Self {
            config,
            circuit_breaker: cb,
            stamina: stam,
            inventory: Arc::new(InventoryManager::default()),
            dailies: Arc::new(DailyManager::default()),
            potions: Arc::new(PotionManager::default()),
            socket,
            gym_manager: Arc::new(Mutex::new(GymManager::new())),
            offering_manager: Arc::new(Mutex::new(OfferingManager::new())),
            state: Arc::new(Mutex::new(PokeState::default())),
            pokemons: Arc::new(Mutex::new(HashMap::new())),
            playable_bosses: Arc::new(Mutex::new(default_playable_bosses())),
            kills: AtomicU64::new(0),
            gold: AtomicU64::new(0),
            exp: AtomicU64::new(0),
            current_level: AtomicU32::new(1),
            running: AtomicBool::new(false),
        }
    }

    pub async fn send_ws_message(&self, msg: &WSMessage) -> Result<(), String> {
        let val = serde_json::to_value(msg).map_err(|e| e.to_string())?;
        self.socket.send_json(&val).await
    }

    pub async fn handle_inbound_message(&self, raw: &str) {
        let msg: WSMessage = match serde_json::from_str(raw) {
            Ok(m) => m,
            Err(_) => return,
        };

        match msg.t.as_str() {
            "pong" => {}
            "welcome" => {
                info!("🎉 Conectado ao PokeIdle! Inicializando automações...");
                if let Some(bosses) = msg.bosses_jogaveis {
                    *self.playable_bosses.lock().await = bosses;
                }
                if let Some(estado) = msg.estado {
                    self.apply_state(estado).await;
                }

                // Configurações iniciais de alto rendimento
                let _ = self.socket.send_json(&serde_json::json!({
                    "t": "cliente.economia",
                    "ativo": true
                })).await;

                let _ = self.send_ws_message(&WSMessage {
                    t: "auto.set".to_string(),
                    auto_ball_sem_parar: Some(true),
                    auto_ball_ate_capturar: Some(false),
                    ball_ids: Some(vec![1, 2, 3, 4]),
                    auto_revive: Some(true),
                    auto_potion: Some(true),
                    ..Default::default()
                }).await;

                let _ = self.send_ws_message(&WSMessage {
                    t: "shop.autoVendaLoot".to_string(),
                    ativo: Some(true),
                    aviso_visto: Some(true),
                    ..Default::default()
                }).await;

                let _ = self.socket.send_json(&serde_json::json!({ "t": "shop.autoLockShiny", "ativo": true })).await;
                let _ = self.socket.send_json(&serde_json::json!({ "t": "shop.autoLockP5", "ativo": true })).await;
                let _ = self.socket.send_json(&serde_json::json!({ "t": "shop.autoLockNota9", "ativo": true, "nota": 8.0 })).await;
                let _ = self.socket.send_json(&serde_json::json!({ "t": "passe.resgatar" })).await;

                // Consulta inicial de ginásios
                let _ = self.send_ws_message(&GymManager::build_query_info_message()).await;
            }
            "estado" => {
                if let Some(estado) = msg.estado {
                    self.apply_state(estado).await;
                }
            }
            "ginasio" => {
                if let Some(info) = msg.info {
                    self.gym_manager.lock().await.update_from_info(&info);
                }
                if let Some(desafio) = msg.desafio {
                    info!("🏛️ [Ginásio {}] Resultado: {}", desafio.tipo, if desafio.venci { "Vitória!" } else { "Derrota." });
                }
            }
            "misticoCaiu" => {
                let st = self.state.lock().await;
                let best_ball = pick_best_ball_for_mystic(&st.balls);
                let _ = self.send_ws_message(&build_mystic_ball_message(best_ball)).await;
                info!("🌌 [Masmorra Mística] Lendário caído! Arremessando Pokébola #{best_ball}!");
            }
            "sessao.renovada" => {
                if let Some(token) = msg.token {
                    info!("🔑 Sessão renovada pelo servidor: {token}");
                }
            }
            _ => {}
        }
    }

    pub async fn apply_state(&self, s: PokeState) {
        if s.level > 0 {
            self.current_level.store(s.level, Ordering::Relaxed);
        }
        if s.gold > 0 {
            self.gold.store(s.gold as u64, Ordering::Relaxed);
        }
        if s.xp > 0 {
            self.exp.store(s.xp as u64, Ordering::Relaxed);
        }

        let mut pks = self.pokemons.lock().await;
        if s.cheio || !s.pokemons.is_empty() {
            if !s.pokemons.is_empty() {
                pks.clear();
                for p in &s.pokemons {
                    pks.insert(p.id, p.clone());
                }
            }
        }
        for p in &s.pk_mud {
            pks.insert(p.id, p.clone());
        }
        for id in &s.pk_fora {
            pks.remove(id);
        }

        *self.state.lock().await = s;
    }

    pub async fn perform_periodic_routine(&self) {
        let st = self.state.lock().await.clone();
        let pks = self.pokemons.lock().await.values().cloned().collect::<Vec<_>>();
        let trainer_level = self.current_level.load(Ordering::Relaxed);

        // 1. Chocadeira / Incubadora
        if let Some(msg) = check_incubator_actions(st.chocadeira.as_ref(), &st.items) {
            info!("🥚 [Chocadeira] Ação de incubação disparada.");
            let _ = self.send_ws_message(&msg).await;
        }

        // 2. Bicicleta
        if let Some(msg) = check_bicycle_actions(st.bicicleta.as_ref(), &st.items) {
            info!("🚲 [Bicicleta] Ação de fabricação/fusão/equipamento disparada.");
            let _ = self.send_ws_message(&msg).await;
        }

        // 3. XP Share na Casa
        if let Some(msg) = check_house_xp_share_actions(st.casa.as_ref(), st.xp_share.as_ref(), &pks, st.active_id) {
            info!("🏠 [Casa XP Share] Alocando Pokémon para treino passivo.");
            let _ = self.send_ws_message(&msg).await;
        }

        // 4. Masmorra Mística
        if let Some(msg) = check_mystic_entry(&st.items, st.mistico.as_ref()) {
            info!("🌌 [Masmorra Mística] Entrando na masmorra mística!");
            let _ = self.send_ws_message(&msg).await;
        }

        // 5. Desafios de Ginásio Elemental
        let mut gm = self.gym_manager.lock().await;
        if gm.should_query_info() {
            let _ = self.send_ws_message(&GymManager::build_query_info_message()).await;
        } else if let Some(gym) = gm.pick_next_gym_to_challenge(trainer_level) {
            info!("🏛️ [Ginásio] Desafiando ginásio de {gym}...");
            let _ = self.send_ws_message(&GymManager::build_gym_challenge_message(&gym)).await;
        }
        drop(gm);

        // 6. Oferenda no Depot
        let mut om = self.offering_manager.lock().await;
        if om.should_run_offering(pks.len()) {
            info!("🎰 [Oferenda] Sacrificando duplicatas fracas no depot.");
            let _ = self.send_ws_message(&OfferingManager::build_offering_message()).await;
        }
        drop(om);

        // 7. Auto-Boss Rush
        let bosses = self.playable_bosses.lock().await;
        let vitorias = st.boss.as_ref().map(|b| b.vitorias.clone()).unwrap_or_default();
        if let Some(boss) = pick_best_boss(&bosses, trainer_level, &vitorias) {
            let tokens = st.boss.as_ref().map(|b| b.tokens).unwrap_or(0);
            if tokens > 0 && !st.boss.as_ref().map(|b| b.auto).unwrap_or(false) {
                info!("👑 [Auto-Boss] Iniciando loop contínuo contra chefe '{}'!", boss.name);
                let _ = self.send_ws_message(&build_boss_auto_message(&boss.key, true)).await;
            }
        }
    }

    pub async fn run_loop(self: Arc<Self>) {
        self.running.store(true, Ordering::SeqCst);
        info!("🚀 Bot [{}] (PokeIdle Online) conectado com protocolo Socket/Engine.IO nativo.", self.config.name);

        let token = &self.config.credentials.token;
        let nick = &self.config.name;

        if !token.is_empty() && token != "seu-token-de-sessao-ou-jwt" {
            let this = Arc::clone(&self);
            let _ = self.socket.connect(move |raw| {
                let this = Arc::clone(&this);
                tokio::spawn(async move {
                    this.handle_inbound_message(&raw).await;
                });
            }).await;

            // Envia handshake hello
            let handshake = WSMessage {
                t: "hello".to_string(),
                nick: Some(nick.clone()),
                token: Some(token.clone()),
                delta: Some(1),
                dispositivo: Some(format!("d{}", chrono::Utc::now().timestamp_millis())),
                ..Default::default()
            };
            let _ = self.send_ws_message(&handshake).await;
        }

        let mut keepalive_timer = tokio::time::interval(Duration::from_secs(20));
        let mut routine_timer = tokio::time::interval(Duration::from_secs(45));

        while self.running.load(Ordering::Relaxed) {
            tokio::select! {
                _ = keepalive_timer.tick() => {
                    if self.socket.is_connected().await {
                        let _ = self.socket.send_json(&serde_json::json!({ "t": "ping" })).await;
                    }
                }
                _ = routine_timer.tick() => {
                    self.perform_periodic_routine().await;
                }
            }
        }
    }
}
