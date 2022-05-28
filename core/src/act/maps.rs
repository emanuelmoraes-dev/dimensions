use crate::ports::models::location::Location;
use crate::ports::traits::t_map::TMap;

pub struct Map {
    pub locations: Vec<Vec<Location>>
}

impl TMap for Map {
    fn move_to(&self, x: usize, y: usize) -> Option<&Location> {
        if let Some(locations) = self.locations.get(y) {
            return locations.get(x);
        }
        None
    }
}
