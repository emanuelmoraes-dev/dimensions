use crate::ports::models::subjects::{SubjectAttr, Player, Npc, NpcRole};
use crate::ports::models::itens::{ItemAttr, Item};

pub trait Describable {
    fn get_title(&self) -> &str;
    fn get_description(&self) -> &str;
}

impl Describable for SubjectAttr {
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
impl<R: NpcRole> Describable for Npc<R> {
    fn get_title(&self) -> &str {
        &self.name
    }
    fn get_description(&self) -> &str {
        &self.description
    }
}

impl Describable for ItemAttr {
    fn get_title(&self) -> &str {
        self.title
    }
    fn get_description(&self) -> &str {
        self.description
    }
}

impl Describable for Item {
    fn get_title(&self) -> &str {
        self.title
    }
    fn get_description(&self) -> &str {
        self.description
    }
}
