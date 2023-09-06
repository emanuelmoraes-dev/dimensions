use std::collections::HashMap;

use crate::ports::models::location::Location;
use crate::ports::traits::t_dimension::TDimension;
use crate::ports::traits::t_id::TOptId;

pub struct Dimension {
    pub id: Option<&'static str>,
    pub locations: HashMap<(i32, i32), Location>,
}

impl TOptId for Dimension {
    fn id(&self) -> Option<&str> {
        self.id
    }
}

impl TDimension for Dimension {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn get_location(&self, x: i32, y: i32) -> Option<&Location> {
        if let Some(location) = self.locations.get(&(x, y)) {
            return Some(location);
        }
        None
    }
    fn add_location(&mut self, x: i32, y: i32, location: Location) {
        self.locations.insert((x, y), location);
    }
}
