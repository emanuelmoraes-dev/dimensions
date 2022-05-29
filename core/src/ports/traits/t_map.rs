use crate::ports::models::location::Location;

use super::t_id::TOptId;

pub trait TMap: TOptId {
    fn move_to(&self, x: usize, y: usize) -> Option<&Location>;
}

pub trait TMapBuilder<Map: TMap> {
    fn generate_map(&self) -> Map;
}
