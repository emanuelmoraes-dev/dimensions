use super::t_role::TNpcRole;
use super::t_dimension::TDimension;
use crate::ports::models::subjects::{Npc, Player};

pub trait TCreator<Dimension: TDimension> {
    fn create_player(&self, nickname: String, description: String) -> Player;
    fn create_dimension(&self) -> Dimension;
    fn create_npc(&self, name: String, description: String, roles: Vec<Box<dyn TNpcRole>>) -> Npc;
}
