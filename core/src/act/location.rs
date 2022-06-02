use crate::ports::models::location_type::LocationType;
use crate::ports::traits::t_id::TId;
use crate::ports::traits::t_location::TLocation;

use super::dimension::Dimension;

pub struct Location {
    pub ltype: LocationType,
    pub teleport_to: Option<Dimension>
}

impl TId for Location {
    fn id(&self) -> &str {
        match self.ltype {
            LocationType::Wall => "Location(Wall)",
            LocationType::Ground => "Location(Ground)",
            LocationType::Gram => "Location(Gram)",
            LocationType::River => "Location(River)",
            LocationType::Cave => "Location(Cave)"
        }
    }
}

impl TLocation for Location {
    fn ltype(&self) -> LocationType {
        self.ltype
    }
}
