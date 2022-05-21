use crate::ports::traits::map::TMap;

pub enum Location {
    Ground,
    River,
    Cave(Box<dyn TMap>)
}
