use std::rc::Rc;

use crate::ports::traits::t_creator::TCreator;
use crate::ports::traits::t_map::TMap;
use crate::ports::models::m_location::Location;
use crate::ports::traits::t_universe::TUniverse;

pub fn get_location<I, D, C>(creator: &C, map: &mut D, x: i32, y: i32, width: u32, height: u32) -> Option<Rc<Location<I>>>
where
    D: TMap<I>,
    C: TCreator<I, D>,
{
    let location = map.get_location(x, y);
    if let Some(location) = location {
        Some(location)
    } else {
        let location = creator.create_location(x, y, width, height);
        map.add_location(x, y, location);
        map.get_location(x, y)
    }
}

pub fn clear_current_map<I, M, U>(universe: &mut U)
where
    M: TMap<I>,
    U: TUniverse<I, M>,
{
    universe.current_map().clear();
}
