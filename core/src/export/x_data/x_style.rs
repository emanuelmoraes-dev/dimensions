use wasm_bindgen::prelude::*;

use crate::assets::fonts::Fonts;
use crate::ports::models::style::{TextStyle, Align};

use super::x_color::XColor;
use super::x_font::XFonts;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum XAlign {
    Start,
    Center,
    End,
}

impl XAlign {
    pub fn to_align(&self) -> Align {
        match self {
            XAlign::Start => Align::Start,
            XAlign::Center => Align::Center,
            XAlign::End => Align::End
        }
    }
}

#[wasm_bindgen]
pub struct XTextStyle {
    pub color: XColor,
    pub font: XFonts,
    pub font_size: f32,
    pub align_x: XAlign,
    pub align_y: XAlign,
    pub offset_x: f32,
    pub offset_y: f32,
}

#[wasm_bindgen]
impl XTextStyle {
    #[wasm_bindgen(constructor)]
    pub fn new(color: &XColor, font: XFonts, font_size: f32, align_x: XAlign, align_y: XAlign, offset_x: f32, offset_y: f32) -> Self {
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
