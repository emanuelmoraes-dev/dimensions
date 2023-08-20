use std::any::Any;

use crate::ports::models::location::Location;

use super::t_id::TOptId;

pub trait TDimension: TOptId {
    fn as_any(&self) -> &dyn Any;
    fn move_to(&self, x: usize, y: usize) -> Option<&Location>;
    fn add_to(&mut self, x: usize, y: usize, location: Location);
}
