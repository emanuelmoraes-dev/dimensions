use image::{Rgba, RgbaImage, ImageBuffer};

use crate::ports::traits::t_gen::TImageGen;

pub struct ImageGen {}

impl ImageGen {
    fn blend_pixels(&self, bg: Rgba<u8>, fg: Rgba<u8>) -> Rgba<u8> {
        let alpha = fg[3] as f32 / 255.0;
        let new_red = (fg[0] as f32 * alpha + bg[0] as f32 * (1.0 - alpha)) as u8;
        let new_green = (fg[1] as f32 * alpha + bg[1] as f32 * (1.0 - alpha)) as u8;
        let new_blue = (fg[2] as f32 * alpha + bg[2] as f32 * (1.0 - alpha)) as u8;
        let new_alpha = (fg[3] + bg[3]) / 2;
        Rgba([new_red, new_green, new_blue, new_alpha])
    }
}

impl TImageGen for ImageGen {
    fn combine(&self, images: Vec<RgbaImage>) -> RgbaImage {
        let mut max_width = 0;
        let mut max_height = 0;
        for image in &images {
            let (width, height) = image.dimensions();
            if width > max_width {
                max_width = width;
            }
            if height > max_height {
                max_height = height;
            }
        }

        let mut combined_image = ImageBuffer::new(max_width, max_height);

        for x in 0..max_width {
            for y in 0..max_height {
                let mut final_pixel = Rgba([0, 0, 0, 0]);

                for image in &images {
                    if x < image.width() && y < image.height() {
                        let pixel = image.get_pixel(x, y);
                        final_pixel = self.blend_pixels(final_pixel, *pixel);
                    }
                }

                combined_image.put_pixel(x, y, final_pixel);
            }
        }

        combined_image
    }
}
