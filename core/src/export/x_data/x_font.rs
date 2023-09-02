use wasm_bindgen::prelude::*;

use crate::assets::fonts::Fonts;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum XFonts {
    RobotoRegular,
    RobotoBold
}

impl XFonts {
    pub fn to_font_data(&self, fonts: &Fonts) -> &'static [u8] {
        match self {
            XFonts::RobotoRegular => fonts.roboto_regular,
            XFonts::RobotoBold => fonts.roboto_bold
        }
    }
}
