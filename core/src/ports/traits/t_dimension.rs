use crate::ports::models::location::Location;

use super::t_id::TOptId;

pub trait TDimension: TOptId {
    fn move_to(&self, x: usize, y: usize) -> Option<&Location>;
}

pub trait TDimensionBuilder<Dimension: TDimension> {
    fn generate_dimension(&self) -> Dimension;
}
