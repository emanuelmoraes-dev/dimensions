use super::feedstock::Feedstock;
use super::roles::NpcRole;
use super::stock::Stock;

pub struct Meta<'a> {
    pub npc_roles: Stock<Feedstock<'a, NpcRole>>,
}
