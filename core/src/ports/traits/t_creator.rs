use super::t_role::TNpcRole;
use crate::ports::models::subjects::{Npc, Player};

pub trait TCreator {
    fn create_player(&self, nickname: String, description: String) -> Player;
    fn create_npc(&self, name: String, description: String, roles: Vec<Box<dyn TNpcRole>>) -> Npc;
}
