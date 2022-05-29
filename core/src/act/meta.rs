use super::feedstock::Feedstock;
use super::stock::Stock;
use super::roles::NpcRole;

pub struct Meta<'a> {
    pub npc_roles: Stock<Feedstock<'a, NpcRole>>
}
