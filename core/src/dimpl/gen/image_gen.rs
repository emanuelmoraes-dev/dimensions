use image::{ImageBuffer, Rgba, RgbaImage};
use rusttype::Font;

use crate::ports::models::style::{TextStyle, AlignEnum};
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
    fn combine(&self, bg_color: &[u8; 4], align_x: &AlignEnum, align_y: &AlignEnum, images: Vec<&RgbaImage>) -> RgbaImage {
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

        for final_x in 0..max_width {
            for final_y in 0..max_height {
                let mut final_pixel = Rgba(bg_color.clone());

                for image in &images {
                    let container_width: f32 = max_width as f32;
                    let container_height: f32 = max_height as f32;
                    let element_width: f32 = image.width() as f32;
                    let element_height: f32 = image.height() as f32;
                    let x: f32 = align_x.get_element_x(container_width, element_width) + final_x as f32;
                    let y: f32 = align_y.get_element_y(container_height, element_height) + final_y as f32;
                    if x >= 0.0 && x < element_width && y >= 0.0 && y < element_height {
                        let pixel = image.get_pixel(x as u32, y as u32);
                        final_pixel = self.blend_pixels(final_pixel, *pixel);
                    }
                }

                combined_image.put_pixel(final_x, final_y, final_pixel);
            }
        }

        combined_image
    }

    fn color(&self, color: &[u8; 4], width: u32, height: u32) -> RgbaImage {
        let color: Rgba<u8> = Rgba(color.clone());
        ImageBuffer::from_pixel(width, height, color)
    }

    fn text(&self, color: &[u8; 4], width: u32, height: u32, text_style: TextStyle, text: &str) -> RgbaImage {
        let mut image = self.color(color, width, height);

        let font = Font::try_from_bytes(text_style.font).unwrap();
        let text_color = Rgba(text_style.color.clone());
        let scale = rusttype::Scale::uniform(text_style.font_size);
        let (text_width, text_height) = text_size(scale, &font, text);
        let x: i32 = (text_style.align_x.get_container_x(width as f32, text_width as f32) + text_style.offset_x) as i32;
        let y: i32 = (text_style.align_y.get_container_y(height as f32, text_height as f32) + text_style.offset_y) as i32;

        draw_text_mut(&mut image, text_color, x as i32, y as i32, scale, &font, text);

        image
    }
}
