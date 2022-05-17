use crate::ports::models::subjects::{Attr, Player, Npc};

pub trait Describable {
    fn get_title(&self) -> &str;
    fn get_description(&self) -> &str;
}

impl Describable for Attr {
    fn get_title(&self) -> &str {
        &self.title
    }
    fn get_description(&self) -> &str {
        &self.description
    }
}

impl Describable for Player {
    fn get_title(&self) -> &str {
        &self.nickname
    }
    fn get_description(&self) -> &str {
        &self.description
    }
}
impl Describable for Npc {
    fn get_title(&self) -> &str {
        &self.name
    }
    fn get_description(&self) -> &str {
        &self.description
    }
}
