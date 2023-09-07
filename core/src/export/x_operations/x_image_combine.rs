use image::RgbaImage;
use wasm_bindgen::prelude::*;

use crate::export::x_core::XCore;
use crate::export::x_data::x_color::XColor;
use crate::export::x_data::x_image::{XImage, XImageFormatEnum};
use crate::export::x_data::x_serialization::x_image_serializalized::XImageSerialized;
use crate::export::x_data::x_style::XAlignEnum;
use crate::ports::traits::t_gen::{TGen, TImageGen};

fn combine_vec(x: &XCore, format: XImageFormatEnum, bg_color: &XColor, align_x: XAlignEnum, align_y: XAlignEnum, images: Vec<&RgbaImage>) -> XImage {
    let bg_color = [bg_color.r, bg_color.g, bg_color.b, bg_color.a];
    let align_x = align_x.to_align();
    let align_y = align_y.to_align();
    let image = x
        .creator
        .gen
        .image()
        .combine(&bg_color, &align_x, &align_y, images);
    XImage::from_image(format, &image)
}

#[wasm_bindgen]
pub struct XImageCombine;

#[wasm_bindgen]
impl XImageCombine {
    #[wasm_bindgen]
    pub fn combine(x: &XCore, format: XImageFormatEnum, bg_color: &XColor, align_x: XAlignEnum, align_y: XAlignEnum, ximages_serialized: Vec<JsValue>) -> Option<XImage> {
        let mut images: Vec<RgbaImage> = Vec::new();
        for js_value in ximages_serialized {
            if let Ok(image) = serde_wasm_bindgen::from_value::<XImageSerialized>(js_value) {
                let image = image.into();
                images.push(image);
            } else {
                return None;
            }
        }
        let images_ref: Vec<&RgbaImage> = images.iter().collect();
        Some(combine_vec(
            x, format, bg_color, align_x, align_y, images_ref,
        ))
    }

    #[wasm_bindgen]
    pub fn combine2(x: &XCore, format: XImageFormatEnum, bg_color: &XColor, align_x: XAlignEnum, align_y: XAlignEnum,
                    i1: &XImage, i2: &XImage) -> XImage {
        let images: Vec<&RgbaImage> = vec![&i1.image, &i2.image];
        combine_vec(x, format, bg_color, align_x, align_y, images)
    }

    #[wasm_bindgen]
    pub fn combine3(x: &XCore, format: XImageFormatEnum, bg_color: &XColor, align_x: XAlignEnum, align_y: XAlignEnum,
                    i1: &XImage, i2: &XImage, i3: &XImage) -> XImage {
        let images: Vec<&RgbaImage> = vec![&i1.image, &i2.image, &i3.image];
        combine_vec(x, format, bg_color, align_x, align_y, images)
    }

    #[wasm_bindgen]
    pub fn combine4(x: &XCore, format: XImageFormatEnum, bg_color: &XColor, align_x: XAlignEnum, align_y: XAlignEnum,
                    i1: &XImage, i2: &XImage, i3: &XImage, i4: &XImage) -> XImage {
        let images: Vec<&RgbaImage> = vec![&i1.image, &i2.image, &i3.image, &i4.image];
        combine_vec(x, format, bg_color, align_x, align_y, images)
    }

    #[wasm_bindgen]
    pub fn combine5(x: &XCore, format: XImageFormatEnum, bg_color: &XColor, align_x: XAlignEnum, align_y: XAlignEnum,
                    i1: &XImage, i2: &XImage, i3: &XImage, i4: &XImage, i5: &XImage) -> XImage {
        let images: Vec<&RgbaImage> = vec![&i1.image, &i2.image, &i3.image, &i4.image, &i5.image];
        combine_vec(x, format, bg_color, align_x, align_y, images)
    }

    #[wasm_bindgen]
    pub fn combine6(x: &XCore, format: XImageFormatEnum, bg_color: &XColor, align_x: XAlignEnum, align_y: XAlignEnum,
                    i1: &XImage, i2: &XImage, i3: &XImage, i4: &XImage, i5: &XImage, i6: &XImage) -> XImage {
        let images: Vec<&RgbaImage> = vec![&i1.image, &i2.image, &i3.image, &i4.image, &i5.image, &i6.image];
        combine_vec(x, format, bg_color, align_x, align_y, images)
    }

    #[wasm_bindgen]
    pub fn combine7(x: &XCore, format: XImageFormatEnum, bg_color: &XColor, align_x: XAlignEnum, align_y: XAlignEnum,
                    i1: &XImage, i2: &XImage, i3: &XImage, i4: &XImage, i5: &XImage, i6: &XImage, i7: &XImage) -> XImage {
        let images: Vec<&RgbaImage> = vec![&i1.image, &i2.image, &i3.image, &i4.image, &i5.image, &i6.image, &i7.image];
        combine_vec(x, format, bg_color, align_x, align_y, images)
    }

    #[wasm_bindgen]
    pub fn combine8(x: &XCore, format: XImageFormatEnum, bg_color: &XColor, align_x: XAlignEnum, align_y: XAlignEnum,
                    i1: &XImage, i2: &XImage, i3: &XImage, i4: &XImage, i5: &XImage, i6: &XImage, i7: &XImage, i8: &XImage) -> XImage {
        let images: Vec<&RgbaImage> = vec![&i1.image, &i2.image, &i3.image, &i4.image, &i5.image, &i6.image, &i7.image, &i8.image];
        combine_vec(x, format, bg_color, align_x, align_y, images)
    }

    #[wasm_bindgen]
    pub fn combine9(x: &XCore, format: XImageFormatEnum, bg_color: &XColor, align_x: XAlignEnum, align_y: XAlignEnum,
                    i1: &XImage, i2: &XImage, i3: &XImage, i4: &XImage, i5: &XImage, i6: &XImage, i7: &XImage, i8: &XImage, i9: &XImage) -> XImage {
        let images: Vec<&RgbaImage> = vec![&i1.image, &i2.image, &i3.image, &i4.image, &i5.image, &i6.image, &i7.image, &i8.image, &i9.image];
        combine_vec(x, format, bg_color, align_x, align_y, images)
    }
}
