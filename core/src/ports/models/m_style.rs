pub enum AlignEnum {
    Start,
    Center,
    End,
}

impl AlignEnum {
    pub fn get_container_x(&self, container_width: f32, element_width: f32) -> f32 {
        match self {
            AlignEnum::Start => 0.0,
            AlignEnum::Center => container_width / 2.0 - element_width / 2.0,
            AlignEnum::End => container_width - element_width
        }
    }

    pub fn get_container_y(&self, container_height: f32, element_heigth: f32) -> f32 {
        match self {
            AlignEnum::Start => 0.0,
            AlignEnum::Center => container_height / 2.0 - element_heigth / 2.0,
            AlignEnum::End => container_height - element_heigth
        }
    }

    pub fn get_element_x(&self, container_width: f32, element_width: f32) -> f32 {
        match self {
            AlignEnum::Start => 0.0,
            AlignEnum::Center => element_width / 2.0 - container_width / 2.0,
            AlignEnum::End => element_width - container_width
        }
    }

    pub fn get_element_y(&self, container_height: f32, element_heigth: f32) -> f32 {
        match self {
            AlignEnum::Start => 0.0,
            AlignEnum::Center => element_heigth / 2.0 - container_height / 2.0,
            AlignEnum::End => element_heigth - container_height
        }
    }
}

pub struct TextStyle {
    pub color: [u8; 4],
    pub font: &'static [u8],
    pub font_size: f32,
    pub align_x: AlignEnum,
    pub align_y: AlignEnum,
    pub offset_x: f32,
    pub offset_y: f32
}
