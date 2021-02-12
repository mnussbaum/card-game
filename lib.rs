use std::collections::HashMap;
use std::fs;

mod player;
use player::Player;

mod card_deck;

mod game_rules;
use game_rules::GameRules;

mod game_state;
pub use game_state::GameState;

mod user_input;

pub fn new_game() -> Result<GameState, String> {
    let yams =
        fs::read_to_string("poo_head_rules.yaml").expect("Something went wrong reading the file");
    let game_rules: GameRules = serde_yaml::from_str(&yams).unwrap();

    // TODO: Get players from user. Use min and max player count from game rules
    let players: Vec<Player> = vec![
        Player {
            name: "Alice".into(),
            hand: HashMap::new(),
        },
        Player {
            name: "Bob".into(),
            hand: HashMap::new(),
        },
    ];

    let mut game_state = GameState::new(game_rules, players);
    game_state.deal();

    return Ok(game_state);
}
