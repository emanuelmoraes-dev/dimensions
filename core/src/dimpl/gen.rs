use image::RgbaImage;

use crate::ports::traits::t_gen::TGen;

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

impl TGen<RgbaImage, ImageGen> for Gen {
    fn image(&self) -> &ImageGen {
        &self.image
    }
}
