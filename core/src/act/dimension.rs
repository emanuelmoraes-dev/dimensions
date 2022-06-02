use crate::ports::models::location::Location;
use crate::ports::traits::t_dimension::TDimension;
use crate::ports::traits::t_id::TOptId;

pub struct Dimension {
    pub id: Option<&'static str>,
    pub locations: Vec<Vec<Location>>,
}

impl TOptId for Dimension {
    fn id(&self) -> Option<&str> {
        self.id
    }
}

impl TDimension for Dimension {
    fn move_to(&self, x: usize, y: usize) -> Option<&Location> {
        if let Some(locations) = self.locations.get(y) {
            return locations.get(x);
        }
        None
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
