use std::any::Any;

use crate::ports::models::location::Location;

use super::t_id::TOptId;

pub trait TDimension: TOptId {
    fn as_any(&self) -> &dyn Any;
    fn get_location(&self, x: i32, y: i32) -> Option<&Location>;
    fn add_location(&mut self, x: i32, y: i32, location: Location);
}
