use wasm_bindgen::prelude::*;

use crate::act::creator::Creator;
use crate::ports::models::subjects::Player;
use crate::ports::traits::t_creator::TCreator;

#[wasm_bindgen]
pub struct Dimensions {
    #[wasm_bindgen(skip)]
    pub creator: Creator,

    #[wasm_bindgen(skip)]
    pub player: Player,
}

#[wasm_bindgen]
impl Dimensions {
    #[wasm_bindgen(constructor)]
    pub fn new(nickname: String, description: String) -> Self {
        let creator = Creator::new();
        let player = creator.create_player(nickname, description);
        Self { creator, player }
    }
}
