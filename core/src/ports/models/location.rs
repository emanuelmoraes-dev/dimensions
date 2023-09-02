use crate::ports::traits::t_dimension::TDimension;

#[derive(Clone, Copy)]
pub enum LocationTypeEnum {
    Ground,
    Cave,
    Wall {
        quantity: u32,
    },
    Gram {
        quantity: u32,
        min_xratio: f32,
        max_xratio: f32,
    },
    River {
        quantity: u32,
        min_xratio: f32,
        max_xratio: f32,
    },
}

pub struct Location {
    pub ltype: LocationTypeEnum,
    pub teleport_to: Option<Box<dyn TDimension>>,
}

impl Location {
    pub fn new(ltype: LocationTypeEnum, teleport_to: Option<Box<dyn TDimension>>) -> Self {
        Self { ltype, teleport_to }
    }
}
