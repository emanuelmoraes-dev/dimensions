use wasm_bindgen::prelude::*;

use crate::{js::console, export::x_data::X};

#[wasm_bindgen]
pub fn x_show_character(x: &X) {
    let player = &x.player;
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
