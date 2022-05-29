use crate::ports::models::location::Location;
use crate::ports::traits::t_id::TOptId;
use crate::ports::traits::t_map::TMap;

pub struct Map {
    pub id: Option<&'static str>,
    pub locations: Vec<Vec<Location>>
}

impl TOptId for Map {
    fn id(&self) -> Option<&str> {
        self.id
    }
}

impl TMap for Map {
    fn move_to(&self, x: usize, y: usize) -> Option<&Location> {
        if let Some(locations) = self.locations.get(y) {
            return locations.get(x);
        }
        None
    }
}
