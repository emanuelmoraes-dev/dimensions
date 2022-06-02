use crate::ports::traits::t_dimension::TDimension;
use crate::ports::traits::t_id::TOptId;

use super::location::Location;

pub struct Dimension {
    pub id: Option<&'static str>,
    pub locations: Vec<Vec<Location>>,
}

impl TOptId for Dimension {
    fn id(&self) -> Option<&str> {
        self.id
    }
}

impl TDimension<Location> for Dimension {
    fn move_to(&self, x: usize, y: usize) -> Option<&Location> {
        if let Some(locations) = self.locations.get(y) {
            return locations.get(x);
        }
        None
    }
}
