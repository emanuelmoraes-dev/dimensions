use wasm_bindgen::prelude::*;

use crate::act::config::Config;
use crate::act::game::Game;
use crate::ports::traits::game::TGame;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct InputRunner {
    nickname: String,
    description: String
}

#[wasm_bindgen]
impl InputRunner {
    #[wasm_bindgen(getter)]
    pub fn nickname(&self) -> String {
        self.nickname.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_nickname(&mut self, nickname: String) {
        self.nickname = nickname;
    }

    #[wasm_bindgen(getter)]
    pub fn description(&self) -> String {
        self.description.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }
}

#[wasm_bindgen]
pub fn run(input: InputRunner) {
    log("Wellcome to Dimensions!");
    let mut game = Game::new(Config::new());
    let nickname = String::from(input.nickname);
    let description = String::from(input.description);
    game.player = Some(game.create_player(nickname, description));
    log("Character created!");
    show_character(&game);
    log("Done!");
}

fn show_character(game: &Game) {
    if let Some(player) = &game.player {
        log(&format!("Nickname: {}", player.nickname));
        log(&format!("Description: {}", player.description));
        log("Attributes:");

        for attr in &player.subject.attrs {
            log(&format!("    {} [points={}, absorb={}]", attr.title, attr.points, attr.absorb));
            log(&format!("        {}", attr.description));
        }
    }
}
