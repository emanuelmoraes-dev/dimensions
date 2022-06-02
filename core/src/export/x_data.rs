use wasm_bindgen::prelude::*;

use crate::act::game::Game;
use crate::ports::models::subjects::Player;
use crate::ports::traits::t_game::TGame;

#[wasm_bindgen]
pub struct Dimensions {
    #[wasm_bindgen(skip)]
    pub game: Game,

    #[wasm_bindgen(skip)]
    pub player: Player
}

#[wasm_bindgen]
impl Dimensions {
    #[wasm_bindgen(constructor)]
    pub fn new(nickname: String, description: String) -> Self {
        let game = Game::new();
        let player = game.create_player(nickname, description);
        Self { game, player }
    }
}
