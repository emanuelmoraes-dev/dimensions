use crate::ports::traits::t_map::TMap;

#[derive(Clone, Copy)]
pub enum LocationTypeEnum {
    Ground,
    Cave,
    Wall,
    Gram,
    River
}

pub struct Location<I> {
    pub ltype: LocationTypeEnum,
    pub image: I,
    pub teleport_to: Option<Box<dyn TMap<I>>>
}

impl<I> Location<I> {
    pub fn new(ltype: LocationTypeEnum, image: I, teleport_to: Option<Box<dyn TMap<I>>>) -> Self {
        Self { ltype, image, teleport_to }
    }
}
