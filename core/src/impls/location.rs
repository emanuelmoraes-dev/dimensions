use crate::ports::models::m_location::Location;
use crate::ports::models::m_location::LocationTypeEnum;
use crate::ports::traits::t_id::TId;

impl TId for LocationTypeEnum {
    fn id(&self) -> &str {
        match self {
            LocationTypeEnum::Ground => "Location(Ground)",
            LocationTypeEnum::Cave => "Location(Cave)",
            LocationTypeEnum::Wall { quantity: _ } => "Location(Wall)",
            LocationTypeEnum::Gram {
                quantity: _,
                min_xratio: _,
                max_xratio: _,
            } => "Location(Gram)",
            LocationTypeEnum::River {
                quantity: _,
                min_xratio: _,
                max_xratio: _,
            } => "Location(River)",
        }
    }
}

impl TId for Location {
    fn id(&self) -> &str {
        self.ltype.id()
    }
}
