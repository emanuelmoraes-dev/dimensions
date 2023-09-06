use image::{DynamicImage, RgbaImage};

pub fn to_rgba_image(image: DynamicImage) -> RgbaImage {
    match image {
        DynamicImage::ImageRgba8(rgba_image) => rgba_image,
        _ => image.to_rgba8(),
    }
}
