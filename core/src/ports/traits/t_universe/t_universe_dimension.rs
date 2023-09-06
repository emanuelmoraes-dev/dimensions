use crate::ports::traits::t_dimension::TDimension;

pub trait TUniverseDimension<Dimension: TDimension> {
    fn move_dimension(&mut self , x: i32, y: i32);
}
