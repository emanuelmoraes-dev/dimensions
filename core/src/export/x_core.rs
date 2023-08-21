use wasm_bindgen::prelude::*;

use crate::dimpl::universe::Universe;

#[wasm_bindgen]
pub struct XCore {
    #[wasm_bindgen(skip)]
    pub universe: Universe
}

#[wasm_bindgen]
impl XCore {
    #[wasm_bindgen(constructor)]
    pub fn new(nickname: String, description: String) -> Self {
        Self {
            universe: Universe::new(nickname, description)
        }
    }
}
