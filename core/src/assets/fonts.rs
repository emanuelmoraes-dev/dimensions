#[derive(Clone)]
pub struct BinaryFont {
    pub data: &'static [u8],
    pub font_size: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl BinaryFont {
    pub fn new(data: &'static [u8], font_size: f32, offset_x: f32, offset_y: f32) -> Self {
        Self {
            data,
            font_size,
            offset_x,
            offset_y,
        }
    }
}

#[derive(Clone)]
pub struct FontsData {
    pub roboto_regular: &'static [u8],
}

impl FontsData {
    pub fn new() -> Self {
        Self {
            roboto_regular: include_bytes!("../../assets/fonts/Roboto/Roboto-Regular.ttf")
        }
    }
}
