use crate::ports::traits::role::TNpcRole;

pub struct NpcRole {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str
}

impl TNpcRole for NpcRole {
    fn get_id(&self) -> &str {
        self.id
    }
    fn get_title(&self) -> &str {
        self.title
    }
    fn get_description(&self) -> &str {
        self.description
    }
}
