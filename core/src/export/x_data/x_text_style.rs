use wasm_bindgen::prelude::*;

use crate::assets::fonts::Fonts;
use crate::ports::models::style::{TextStyle, TextAlign};

use super::x_color::XColor;
use super::x_font::XFonts;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum XTextAlign {
    Left,
    Center,
    Right,
}

impl XTextAlign {
    pub fn to_text_align(&self) -> TextAlign {
        match self {
            XTextAlign::Left => TextAlign::Left,
            XTextAlign::Center => TextAlign::Center,
            XTextAlign::Right => TextAlign::Right
        }
    }
}

#[wasm_bindgen]
pub struct XTextStyle {
    pub color: XColor,
    pub font: XFonts,
    pub font_size: f32,
    pub align_x: XTextAlign,
    pub align_y: XTextAlign,
    pub offset_x: f32,
    pub offset_y: f32,
}

#[wasm_bindgen]
impl XTextStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(color: XColor, font: XFonts, font_size: f32, align_x: XTextAlign, align_y: XTextAlign, offset_x: f32, offset_y: f32) -> Self {
        Self { color, font, font_size, align_x, align_y, offset_x, offset_y }
    }
}

impl XTextStyle {
    pub fn to_text_style(&self, fonts: &Fonts) -> TextStyle {
        TextStyle {
            color: [self.color.r, self.color.g, self.color.b, self.color.a],
            font: self.font.to_font_data(fonts),
            font_size: self.font_size,
            align_x: self.align_x.to_text_align(),
            align_y: self.align_y.to_text_align(),
            offset_x: self.offset_x,
            offset_y: self.offset_y,
        }
    }
}
