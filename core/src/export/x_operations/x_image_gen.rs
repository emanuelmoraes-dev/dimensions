use image::RgbaImage;
use wasm_bindgen::prelude::*;

use crate::export::x_core::XCore;
use crate::export::x_data::x_color::XColor;
use crate::export::x_data::x_image::{XImage, XImageFormat};
use crate::export::x_data::x_text_style::XTextStyle;
use crate::ports::traits::t_gen::{TGen, TImageGen};

#[wasm_bindgen]
pub struct XImageGen;

#[wasm_bindgen]
impl XImageGen {
    #[wasm_bindgen]
    pub fn combine2(x: &XCore, format: XImageFormat, i1: XImage, i2: XImage) -> Option<XImage> {
        let images: Vec<RgbaImage> = vec![i1.image, i2.image];
        let image = x.universe.gen.image().combine(images);
        XImage::from_image(format, image)
    }

    #[wasm_bindgen]
    pub fn color(x: &XCore, format: XImageFormat, color: XColor, width: u32, height: u32) -> Option<XImage> {
        let color = [color.r, color.g, color.b, color.a];
        let image = x.universe.gen.image().color(color, width, height);
        XImage::from_image(format, image)
    }

    #[wasm_bindgen]
    pub fn text(x: &XCore, format: XImageFormat, color: XColor, width: u32, height: u32, text_style: XTextStyle, text: String) -> Option<XImage> {
        let color = [color.r, color.g, color.b, color.a];
        let text_style = text_style.to_text_style(&x.universe.config.fonts);
        let image = x
            .universe
            .gen
            .image()
            .text(color, width, height, text_style, &text);
        XImage::from_image(format, image)
    }
}
