use crate::ports::models::subjects::{Player, Npc};
use super::t_role::TNpcRole;

pub trait TGame {
    fn create_player(&self, nickname: String, description: String) -> Player;
    fn create_npc(&self, name: String, description: String, roles: Vec<Box<dyn TNpcRole>>) -> Npc;
}
