use image::{ImageBuffer, Rgba, RgbaImage};
use rusttype::Font;

use crate::ports::models::style::TextStyle;
use crate::ports::traits::t_gen::TImageGen;

use imageproc::drawing::{draw_text_mut, text_size};

pub struct ImageGen;

impl ImageGen {
    fn blend_pixels(&self, bg: Rgba<u8>, fg: Rgba<u8>) -> Rgba<u8> {
        let alpha = fg[3] as f32 / 255.0;
        let new_red = (fg[0] as f32 * alpha + bg[0] as f32 * (1.0 - alpha)) as u8;
        let new_green = (fg[1] as f32 * alpha + bg[1] as f32 * (1.0 - alpha)) as u8;
        let new_blue = (fg[2] as f32 * alpha + bg[2] as f32 * (1.0 - alpha)) as u8;
        Rgba([new_red, new_green, new_blue, bg[3]])
    }
}

impl TImageGen<RgbaImage> for ImageGen {
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
                let mut final_pixel = Rgba([0, 0, 0, 255]);

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

    fn color(&self, color: [u8; 4], width: u32, height: u32) -> RgbaImage {
        let color: Rgba<u8> = Rgba(color);
        ImageBuffer::from_pixel(width, height, color)
    }

    fn text(&self, color: [u8; 4], width: u32, height: u32, text_style: TextStyle, text: &str) -> RgbaImage {
        let mut image = self.color(color, width, height);

        let font = Font::try_from_bytes(text_style.font).unwrap();
        let text_color = Rgba(text_style.color);
        let scale = rusttype::Scale::uniform(text_style.font_size);
        let (text_width, text_height) = text_size(scale, &font, text);
        let x: i32 = (text_style.align_x.get_x(width as f32, text_width as f32) + text_style.offset_x) as i32;
        let y: i32 = (text_style.align_y.get_y(height as f32, text_height as f32) + text_style.offset_y) as i32;

        draw_text_mut(&mut image, text_color, x as i32, y as i32, scale, &font, text);

        image
    }
}
