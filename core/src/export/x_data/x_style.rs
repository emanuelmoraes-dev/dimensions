use wasm_bindgen::prelude::*;

use crate::assets::fonts::Fonts;
use crate::ports::models::m_style::{TextStyle, AlignEnum};

use super::x_color::XColor;
use super::x_font::XFontsEnum;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum XAlignEnum {
    Start,
    Center,
    End,
}

impl XAlignEnum {
    pub fn to_align(&self) -> AlignEnum {
        match self {
            XAlignEnum::Start => AlignEnum::Start,
            XAlignEnum::Center => AlignEnum::Center,
            XAlignEnum::End => AlignEnum::End
        }
    }
}

#[wasm_bindgen]
pub struct XTextStyle {
    pub color: XColor,
    pub font: XFontsEnum,
    pub font_size: f32,
    pub align_x: XAlignEnum,
    pub align_y: XAlignEnum,
    pub offset_x: f32,
    pub offset_y: f32,
}

#[wasm_bindgen]
impl XTextStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(color: &XColor, font: XFontsEnum, font_size: f32, align_x: XAlignEnum, align_y: XAlignEnum, offset_x: f32, offset_y: f32) -> Self {
        Self { color: color.clone(), font, font_size, align_x, align_y, offset_x, offset_y }
    }
}

impl XTextStyle {
    pub fn to_text_style(&self, fonts: &Fonts) -> TextStyle {
        TextStyle {
            color: [self.color.r, self.color.g, self.color.b, self.color.a],
            font: self.font.to_font_data(fonts),
            font_size: self.font_size,
            align_x: self.align_x.to_align(),
            align_y: self.align_y.to_align(),
            offset_x: self.offset_x,
            offset_y: self.offset_y,
        }
    }
}
