use std::collections::HashMap;

use crate::ports::models::location::Location;
use crate::ports::traits::t_dimension::TDimension;
use crate::ports::traits::t_id::TOptId;

pub struct Dimension {
    pub id: Option<&'static str>,
    pub locations: HashMap<(usize, usize), Location>,
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
    fn move_to(&self, x: usize, y: usize) -> Option<&Location> {
        if let Some(location) = self.locations.get(&(x, y)) {
            return Some(location);
        }
        None
    }
    fn add_to(&mut self, x: usize, y: usize, location: Location) {
        self.locations.insert((x, y), location);
    }
}
