use std::collections::HashMap;
use std::rc::Rc;

use image::RgbaImage;

use crate::ports::models::m_location::Location;
use crate::ports::traits::t_map::TMap;
use crate::ports::traits::t_id::TOptId;

pub struct Map {
    pub id: Option<&'static str>,
    pub locations: HashMap<(i32, i32), Rc<Location<RgbaImage>>>,
}

impl Map {
    pub fn new(id: Option<&'static str>) -> Map {
        let locations = HashMap::new();
        Map { id, locations }
    }
}

impl TOptId for Map {
    fn id(&self) -> Option<&str> {
        self.id
    }
}

impl TMap<RgbaImage> for Map {
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
    fn remove_location(&mut self, x: i32, y: i32) -> Option<Rc<Location<RgbaImage>>> {
        self.locations.remove(&(x, y))
    }
    fn clear(&mut self) {
        self.locations.clear();
    }
}
