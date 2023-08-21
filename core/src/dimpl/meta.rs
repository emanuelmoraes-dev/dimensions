use crate::ports::models::location::LocationType;
use crate::ports::traits::t_meta::TMeta;

use super::dimension::Dimension;
use super::feedstock::Feedstock;
use super::roles::NpcRole;
use super::stock::Stock;

pub struct Meta {
    pub location_types: Stock<Feedstock<LocationType>>,
    pub npc_roles: Stock<Feedstock<NpcRole>>,
}

impl Meta {
    pub fn new() -> Self {
        let location_types: Stock<Feedstock<LocationType>> = Stock::new();
        let npc_roles: Stock<Feedstock<NpcRole>> = Stock::new();
        Self {
            location_types,
            npc_roles,
        }
    }
}

impl TMeta<Dimension> for Meta {
    fn generate_dimension(&self) -> Dimension {
        panic!("Not implemented")
    }
}
