use crate::ports::models::location::Location;

pub trait TMap {
    fn move_to(&self, x: usize, y: usize) -> Option<&Location>;
}

pub trait TMapBuilder<Map: TMap> {
    fn generate_map(&self) -> Map;
}
