use crate::ports::models::location_type::LocationType;
use crate::ports::traits::t_id::TId;
use crate::ports::traits::t_location::TLocation;

use super::dimension::Dimension;

impl TId for LocationType {
    fn id(&self) -> &str {
        match self {
            LocationType::Ground => "Location(Ground)",
            LocationType::Cave => "Location(Cave)",
            LocationType::Wall {
                quantity: _
            } => "Location(Wall)",
            LocationType::Gram {
                quantity: _,
                min_xratio: _,
                max_xratio: _
            } => "Location(Gram)",
            LocationType::River {
                quantity: _,
                min_xratio: _,
                max_xratio: _
            } => "Location(River)",
        }
    }
}

pub struct Location {
    pub ltype: LocationType,
    pub teleport_to: Option<Dimension>,
}

impl TId for Location {
    fn id(&self) -> &str {
        self.ltype.id()
    }
}

impl TLocation for Location {
    fn ltype(&self) -> LocationType {
        self.ltype
    }
}
