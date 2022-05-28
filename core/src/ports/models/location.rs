use crate::ports::traits::t_map::TMap;

pub enum Location {
    Wall,
    Ground,
    River,
    Cave(Box<dyn TMap>)
}
