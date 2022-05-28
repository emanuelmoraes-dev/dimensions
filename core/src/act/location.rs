use crate::ports::traits::t_id::TId;
use crate::ports::models::location::Location;

impl TId for Location {
    fn id(&self) -> &str {
        match self {
            Location::Wall => "Location(Wall)",
            Location::Ground => "Location(Ground)",
            Location::River => "Location(River)",
            Location::Cave(_) => "Location(Cave)"
        }
    }
}
