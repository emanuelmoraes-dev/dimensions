use image::{Rgba, RgbaImage};
use wasm_bindgen::prelude::*;

use crate::export::x_core::XCore;
use crate::export::x_data::x_font::XFont;
use crate::export::x_data::x_image::{XImage, XImageFormat};
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
    pub fn color(x: &XCore, format: XImageFormat, color: &[u8], width: u32, height: u32) -> Option<XImage> {
        let color: Rgba<u8> = Rgba([color[0], color[1], color[2], color[3]]);
        let image = x.universe.gen.image().color(color, width, height);
        XImage::from_image(format, image)
    }

    #[wasm_bindgen]
    pub fn text(x: &XCore, format: XImageFormat, font: XFont, bg_color: &[u8], text_color: &[u8], text: String, width: u32, height: u32) -> Option<XImage> {
        let bg_color: Rgba<u8> = Rgba([bg_color[0], bg_color[1], bg_color[2], bg_color[3]]);
        let text_color: Rgba<u8> =
            Rgba([text_color[0], text_color[1], text_color[2], text_color[3]]);
        let font = font.binary_font(&x.universe.config.fonts);
        let image = x
            .universe
            .gen
            .image()
            .text(font, bg_color, text_color, &text, width, height);
        XImage::from_image(format, image)
    }
}
