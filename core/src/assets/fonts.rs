pub struct BinaryFont {
    pub data: &'static [u8],
    pub font_size: f32,
    pub offset_x: f32,
    pub offset_y: f32
}

impl BinaryFont {
    pub fn new(data: &'static [u8], font_size: f32, offset_x: f32, offset_y: f32) -> Self {
        Self {data, font_size, offset_x, offset_y}
    }
}

pub struct Fonts {
    pub roboto_regular: BinaryFont
}

impl Fonts {
    pub fn new() -> Fonts {
        Fonts {
            roboto_regular: BinaryFont::new(include_bytes!("../../assets/fonts/Roboto/Roboto-Regular.ttf"), 12.0, 0.0, 0.0)
        }
    }
}
