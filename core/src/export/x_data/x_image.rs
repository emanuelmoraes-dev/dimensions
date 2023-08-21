use wasm_bindgen::prelude::*;

use std::io::{BufWriter, Cursor};

use image::{ImageOutputFormat, RgbaImage};

use crate::util::image_util::to_rgba_image;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum XImageFormat {
    Png,
}

pub fn get_image_output_format(format: XImageFormat) -> ImageOutputFormat {
    match format {
        XImageFormat::Png => ImageOutputFormat::Png,
    }
}

#[wasm_bindgen]
pub struct XImage {
    #[wasm_bindgen(skip)]
    pub image: RgbaImage,

    #[wasm_bindgen(skip)]
    pub bytes: Vec<u8>,

    pub format: XImageFormat,
}

#[wasm_bindgen]
impl XImage {
    #[wasm_bindgen(constructor)]
    pub fn from_bytes(format: XImageFormat, bytes: Vec<u8>) -> Option<XImage> {
        let cursor = Cursor::new(&bytes);
        let result = image::load(cursor, image::ImageFormat::Png);
        let image: RgbaImage = match result {
            Ok(image) => to_rgba_image(image),
            Err(_) => return None,
        };
        Some(Self {
            image,
            bytes,
            format,
        })
    }

    #[wasm_bindgen]
    pub fn data(&self) -> Vec<u8> {
        self.bytes.clone()
    }
}

impl XImage {
    pub fn from_image(format: XImageFormat, image: RgbaImage) -> Option<XImage> {
        let image = image::DynamicImage::ImageRgba8(image);
        let mut bytes = Vec::new();

        {
            let cursor = Cursor::new(&mut bytes);
            let mut writer = BufWriter::new(cursor);
            let format = get_image_output_format(format);
            let result = image.write_to(&mut writer, format);
            if let Err(_) = result {
                return None;
            }
        }

        let image = to_rgba_image(image);
        Some(Self {
            image,
            bytes,
            format,
        })
    }
}
