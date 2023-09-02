use wasm_bindgen::prelude::*;

use crate::assets::fonts::Fonts;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum XFontsEnum {
    RobotoRegular,
    RobotoBold
}

impl XFontsEnum {
    pub fn to_font_data(&self, fonts: &Fonts) -> &'static [u8] {
        match self {
            XFontsEnum::RobotoRegular => fonts.roboto_regular,
            XFontsEnum::RobotoBold => fonts.roboto_bold
        }
    }
}
