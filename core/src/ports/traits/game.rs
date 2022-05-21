use crate::ports::models::subjects::{Player, Npc};

use super::role::TNpcRole;

pub trait TGame<Role: TNpcRole> {
    fn create_player(&self, nickname: String, description: String) -> Player;
    fn create_npc(&self, name: String, description: String, roles: Vec<Role>) -> Npc<Role>;
}
