use bot_pokeidle_rust::game::features::*;
use bot_pokeidle_rust::game::gyms::{GymManager, GYM_TYPES};
use bot_pokeidle_rust::game::types::*;
use std::collections::HashMap;

#[test]
fn test_pokeidle_wsmessage_handshake() {
    let msg = WSMessage {
        t: "hello".to_string(),
        nick: Some("Red".to_string()),
        token: Some("sample-jwt-token".to_string()),
        delta: Some(1),
        dispositivo: Some("d1700000000".to_string()),
        ..Default::default()
    };

    let serialized = serde_json::to_string(&msg).unwrap();
    assert!(serialized.contains(r#""t":"hello""#));
    assert!(serialized.contains(r#""nick":"Red""#));
    assert!(serialized.contains(r#""token":"sample-jwt-token""#));
}

#[test]
fn test_pokeidle_gym_manager_18_types() {
    assert_eq!(GYM_TYPES.len(), 18);
    let mut gm = GymManager::new();
    assert!(gm.should_query_info());

    let info = PokeGinasioInfo {
        pode_registrar: true,
        nivel_min: 20,
        ginasios: vec![
            PokeGinasioEntry {
                tipo: "fogo".to_string(),
                sou_lider: true,
                lider_nick: "Red".to_string(),
            },
            PokeGinasioEntry {
                tipo: "agua".to_string(),
                sou_lider: false,
                lider_nick: "Misty".to_string(),
            },
        ],
    };

    gm.update_from_info(&info);
    assert!(!gm.should_query_info());

    // Below min level
    assert_eq!(gm.pick_next_gym_to_challenge(15), None);

    // Eligible level: will challenge a non-leader gym
    let target = gm.pick_next_gym_to_challenge(25);
    assert!(target.is_some());
}

#[test]
fn test_pokeidle_features_logic() {
    // 1. Mystic ball priority
    let mut balls = HashMap::new();
    balls.insert("1".to_string(), 100);
    balls.insert("3".to_string(), 5);
    balls.insert("4".to_string(), 2);
    // Best available is Ultra Ball (4)
    assert_eq!(pick_best_ball_for_mystic(&balls), 4);

    // 2. Incubator shiny craft with 10 fragments
    let mut items = HashMap::new();
    items.insert("70012".to_string(), 12);
    let action = check_incubator_actions(None, &items);
    assert!(action.is_some());
    assert_eq!(action.unwrap().t, "ovoShiny.fabricar");

    // 3. Bicycle action with 10 fragments
    items.insert("70013".to_string(), 10);
    let action = check_bicycle_actions(None, &items);
    assert!(action.is_some());
    assert_eq!(action.unwrap().t, "bicicleta.fabricar");

    // 4. Boss selection
    let bosses = default_playable_bosses();
    let wins = HashMap::new();
    let best = pick_best_boss(&bosses, 40, &wins);
    assert!(best.is_some());
    assert_eq!(best.unwrap().key, "erika"); // Erika requires level 40 <= 40
}
