use crate::ports::models::location::LocationType;

use super::feedstock::Feedstock;
use super::roles::NpcRole;
use super::stock::Stock;

pub struct Meta<'a> {
    pub location_types: Stock<Feedstock<'a, LocationType>>,
    pub npc_roles: Stock<Feedstock<'a, NpcRole>>,
}
