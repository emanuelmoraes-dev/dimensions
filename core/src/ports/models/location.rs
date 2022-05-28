use crate::ports::traits::map::TMap;

pub enum Location {
    Wall,
    Ground,
    River,
    Cave(Box<dyn TMap>)
}
