use image::RgbaImage;

use crate::ports::models::m_location::Location;
use crate::ports::models::m_location::LocationTypeEnum;
use crate::ports::traits::t_id::TId;

impl LocationTypeEnum {
    pub fn color(&self) -> [u8; 4] {
        match self {
            LocationTypeEnum::Ground => [139, 69, 19, 255],
            LocationTypeEnum::Cave => [105, 105, 105, 255],
            LocationTypeEnum::Wall => [128, 128, 128, 255],
            LocationTypeEnum::Gram => [0, 128, 0, 255],
            LocationTypeEnum::River => [173, 216, 230, 255],
        }
    }
}

impl TId for LocationTypeEnum {
    fn id(&self) -> &str {
        match self {
            LocationTypeEnum::Ground => "Ground",
            LocationTypeEnum::Cave => "Cave",
            LocationTypeEnum::Wall => "Wall",
            LocationTypeEnum::Gram => "Gram",
            LocationTypeEnum::River => "River"
        }
    }
}

impl TId for Location<RgbaImage> {
    fn id(&self) -> &str {
        self.ltype.id()
    }
}
