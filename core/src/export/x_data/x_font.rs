use wasm_bindgen::prelude::*;

use crate::assets::fonts::{BinaryFont, FontsData};

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum XFontsData {
    RobotoRegular
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct  XFont {
    pub data: XFontsData,
    pub font_size: f32,
    pub offset_x: f32,
    pub offset_y: f32
}

#[wasm_bindgen]
impl XFont {
    #[wasm_bindgen(constructor)]
    pub fn new(data: XFontsData, font_size: f32, offset_x: f32, offset_y: f32) -> Self {
        Self {
            data,
            font_size,
            offset_x,
            offset_y
        }
    }
}

impl XFont {
    pub fn binary_font(&self, fonts: &FontsData) -> BinaryFont {
        match self.data {
            XFontsData::RobotoRegular => BinaryFont::new(fonts.roboto_regular, self.font_size, self.offset_x, self.offset_y)
        }
    }
}
