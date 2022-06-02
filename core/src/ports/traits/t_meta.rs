use super::t_dimension::TDimension;

pub trait TMeta<Dimension: TDimension> {
    fn generate_dimension(&self) -> Dimension;
}
