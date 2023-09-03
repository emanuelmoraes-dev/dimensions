use image::RgbaImage;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct XImageSerialized {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32
}

impl From<RgbaImage> for XImageSerialized {
    fn from(image: RgbaImage) -> Self {
        let (width, height) = image.dimensions();
        let data = image.into_raw();
        XImageSerialized { data, width, height }
    }
}

impl Into<RgbaImage> for XImageSerialized {
    fn into(self) -> RgbaImage {
        RgbaImage::from_raw(self.width, self.height, self.data).unwrap()
    }
}
