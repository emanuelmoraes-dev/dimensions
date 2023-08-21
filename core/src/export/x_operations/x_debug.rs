use wasm_bindgen::prelude::*;

use crate::{export::x_core::XCore, js::console};

#[wasm_bindgen]
pub struct XDebug;

#[wasm_bindgen]
impl XDebug {
    #[wasm_bindgen]
    pub fn show_character(x: &XCore) {
        let player = &x.universe.player;
        console::log(&format!("Nickname: {}", player.nickname));
        console::log(&format!("Description: {}", player.description));
        console::log("Attributes:");

        for attr in &player.subject.attrs {
            console::log(&format!(
                "    {} [points={}, absorb={}]",
                attr.title, attr.points, attr.absorb
            ));
            console::log(&format!("        {}", attr.description));
        }
    }
}
