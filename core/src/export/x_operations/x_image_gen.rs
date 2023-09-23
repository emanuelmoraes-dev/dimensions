use std::rc::Rc;

use image::RgbaImage;
use wasm_bindgen::prelude::*;

use crate::export::x_core::XCore;
use crate::export::x_data::x_color::XColor;
use crate::export::x_data::x_image::{XImage, XImageFormatEnum};
use crate::export::x_data::x_style::XTextStyle;
use crate::ports::traits::t_gen::{TGen, TImageGen};

#[wasm_bindgen]
pub struct XImageGen;

#[wasm_bindgen]
impl XImageGen {
    #[wasm_bindgen]
    pub fn color(x: &XCore, format: XImageFormatEnum, color: &XColor, width: u32, height: u32) -> XImage {
        let color = [color.r, color.g, color.b, color.a];
        let image: RgbaImage = x.creator.gen.image().color(&color, width, height);
        let image = Rc::new(image);
        XImage::from_image(format, image)
    }

    #[wasm_bindgen]
    pub fn text(x: &XCore, format: XImageFormatEnum, color: &XColor, width: u32, height: u32, text_style: &XTextStyle, text: String) -> XImage {
        let color = [color.r, color.g, color.b, color.a];
        let text_style = text_style.to_text_style(&x.creator.config.fonts);
        let image: RgbaImage = x.creator.gen.image().text(&color, width, height, text_style, &text);
        let image: Rc<RgbaImage> = Rc::new(image);
        XImage::from_image(format, image)
    }
}
