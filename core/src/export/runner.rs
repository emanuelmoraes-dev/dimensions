use wasm_bindgen::prelude::*;

use crate::ports::models::subjects::Player;
use super::data::Dimensions;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run(dimensions: &Dimensions) {
    log("Wellcome to Dimensions!");
    log("Character created!");
    show_character(&dimensions.player);
    log("Done!");
}

fn show_character(player: &Player) {
    log(&format!("Nickname: {}", player.nickname));
    log(&format!("Description: {}", player.description));
    log("Attributes:");

    for attr in &player.subject.attrs {
        log(&format!("    {} [points={}, absorb={}]", attr.title, attr.points, attr.absorb));
        log(&format!("        {}", attr.description));
    }
}
