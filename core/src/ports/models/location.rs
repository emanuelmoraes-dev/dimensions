use crate::ports::traits::t_dimension::TDimension;

pub enum Location {
    Wall,
    Ground,
    River,
    Cave(Box<dyn TDimension>)
}
