use crate::ports::models::location::Location;
use crate::ports::models::location::LocationType;
use crate::ports::traits::t_id::TId;

impl TId for LocationType {
    fn id(&self) -> &str {
        match self {
            LocationType::Ground => "Location(Ground)",
            LocationType::Cave => "Location(Cave)",
            LocationType::Wall { quantity: _ } => "Location(Wall)",
            LocationType::Gram {
                quantity: _,
                min_xratio: _,
                max_xratio: _,
            } => "Location(Gram)",
            LocationType::River {
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
