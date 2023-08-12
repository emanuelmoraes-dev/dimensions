use wasm_bindgen::prelude::*;

use crate::operations::runner;

use super::x_data::X;

#[wasm_bindgen]
pub fn xrun(x: &X) {
    runner::run(&x.player);
}
