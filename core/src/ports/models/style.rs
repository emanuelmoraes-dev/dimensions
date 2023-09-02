pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl TextAlign {
    pub fn get_x(&self, image_width: f32, text_width: f32) -> f32 {
        match self {
            TextAlign::Left => 0.0,
            TextAlign::Center => image_width / 2.0 - text_width / 2.0,
            TextAlign::Right => image_width - text_width
        }
    }

    pub fn get_y(&self, image_height: f32, text_width: f32) -> f32 {
        match self {
            TextAlign::Left => 0.0,
            TextAlign::Center => image_height / 2.0 - text_width / 2.0,
            TextAlign::Right => image_height - text_width
        }
    }
}

pub struct TextStyle {
    pub color: [u8; 4],
    pub font: &'static [u8],
    pub font_size: f32,
    pub align_x: TextAlign,
    pub align_y: TextAlign,
    pub offset_x: f32,
    pub offset_y: f32
}
