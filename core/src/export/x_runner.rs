use wasm_bindgen::prelude::*;

use crate::operations::runner;

use super::x_data::Dimensions;

#[wasm_bindgen]
pub fn run(dimensions: &Dimensions) {
    runner::run(&dimensions.player);
}
