use image::RgbaImage;
use wasm_bindgen::prelude::*;

use crate::export::x_core::XCore;
use crate::export::x_data::x_color::XColor;
use crate::export::x_data::x_image::{XImage, XImageFormat};
use crate::export::x_data::x_style::{XTextStyle, XAlign};
use crate::ports::traits::t_gen::{TGen, TImageGen};

#[wasm_bindgen]
pub struct XImageGen;

#[wasm_bindgen]
impl XImageGen {
    #[wasm_bindgen]
    pub fn combine2(x: &XCore, format: XImageFormat, bg_color: &XColor, align_x: XAlign, align_y: XAlign, i1: XImage, i2: XImage) -> Option<XImage> {
        let bg_color = [bg_color.r, bg_color.g, bg_color.b, bg_color.a];
        let align_x = align_x.to_align();
        let align_y = align_y.to_align();
        let images: Vec<RgbaImage> = vec![i1.image, i2.image];
        let image = x.universe.gen.image().combine(&bg_color, &align_x, &align_y, images);
        XImage::from_image(format, image)
    }

    #[wasm_bindgen]
    pub fn color(x: &XCore, format: XImageFormat, color: &XColor, width: u32, height: u32) -> Option<XImage> {
        let color = [color.r, color.g, color.b, color.a];
        let image = x.universe.gen.image().color(&color, width, height);
        XImage::from_image(format, image)
    }

    #[wasm_bindgen]
    pub fn text(x: &XCore, format: XImageFormat, color: &XColor, width: u32, height: u32, text_style: &XTextStyle, text: String) -> Option<XImage> {
        let color = [color.r, color.g, color.b, color.a];
        let text_style = text_style.to_text_style(&x.universe.config.fonts);
        let image = x
            .universe
            .gen
            .image()
            .text(&color, width, height, text_style, &text);
        XImage::from_image(format, image)
    }
}
