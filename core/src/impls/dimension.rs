use std::collections::HashMap;
use std::rc::Rc;

use image::RgbaImage;

use crate::ports::models::m_location::Location;
use crate::ports::traits::t_dimension::TDimension;
use crate::ports::traits::t_id::TOptId;

pub struct Dimension {
    pub id: Option<&'static str>,
    pub locations: HashMap<(i32, i32), Rc<Location<RgbaImage>>>,
}

impl Dimension {
    pub fn new(id: Option<&'static str>) -> Dimension {
        let locations = HashMap::new();
        Dimension { id, locations }
    }
}

impl TOptId for Dimension {
    fn id(&self) -> Option<&str> {
        self.id
    }
}

impl TDimension<RgbaImage> for Dimension {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn get_location(&self, x: i32, y: i32) -> Option<Rc<Location<RgbaImage>>> {
        if let Some(location) = self.locations.get(&(x, y)) {
            return Some(Rc::clone(location));
        }
        None
    }
    fn add_location(&mut self, x: i32, y: i32, location: Location<RgbaImage>) {
        self.locations.insert((x, y), Rc::from(location));
    }
}
