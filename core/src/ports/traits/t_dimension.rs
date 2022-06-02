use super::t_id::TOptId;
use super::t_location::TLocation;

pub trait TDimension<Location: TLocation>: TOptId {
    fn move_to(&self, x: usize, y: usize) -> Option<&Location>;
}

pub trait TDimensionBuilder<Location: TLocation, Dimension: TDimension<Location>> {
    fn generate_dimension(&self) -> Dimension;
}
