use super::t_role::TNpcRole;
use super::t_dimension::TDimension;
use crate::ports::models::{m_subjects::{Npc, Player}, m_location::Location};

pub trait TCreator<I, Dimension: TDimension<I>> {
    fn create_player(&self, nickname: String, description: String) -> Player;
    fn create_dimension(&self) -> Dimension;
    fn create_location(&self, x: i32, y: i32, width: u32, height: u32) -> Location<I>;
    fn create_npc(&self, name: String, description: String, roles: Vec<Box<dyn TNpcRole>>) -> Npc;
}
