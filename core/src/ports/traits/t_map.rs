use std::{any::Any, rc::Rc};

use crate::ports::models::m_location::Location;

use super::t_id::TOptId;

pub trait TMap<I>: TOptId {
    fn as_any(&self) -> &dyn Any;
    fn get_location(&self, x: i32, y: i32) -> Option<Rc<Location<I>>>;
    fn add_location(&mut self, x: i32, y: i32, location: Location<I>);
}
