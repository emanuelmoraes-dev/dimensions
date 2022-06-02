use crate::ports::models::location_type::LocationType;

pub trait TLocation {
    fn ltype(&self) -> LocationType;
}
