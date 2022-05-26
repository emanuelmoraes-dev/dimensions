use crate::ports::traits::role::TNpcRole;

pub struct NpcRole {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str
}

impl TNpcRole for NpcRole {
    fn id(&self) -> &str {
        self.id
    }
    fn title(&self) -> &str {
        self.title
    }
    fn description(&self) -> &str {
        self.description
    }
}
