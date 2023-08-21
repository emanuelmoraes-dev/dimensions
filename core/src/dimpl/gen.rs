use image::{Rgba, RgbaImage};

use crate::{assets::fonts::BinaryFont, ports::traits::t_gen::TGen};

use self::image_gen::ImageGen;

pub mod image_gen;

pub struct Gen {
    image: ImageGen,
}

impl Gen {
    pub fn new() -> Gen {
        Gen { image: ImageGen }
    }
}

impl TGen<RgbaImage, Rgba<u8>, BinaryFont, ImageGen> for Gen {
    fn image(&self) -> &ImageGen {
        &self.image
    }
}
