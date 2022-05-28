use crate::ports::traits::{t_role::TNpcRole, t_id::TId};

pub struct NpcRole {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str
}

impl TId for NpcRole {
    fn id(&self) -> &str {
        self.id
    }
}

impl TNpcRole for NpcRole {
    fn title(&self) -> &str {
        self.title
    }
    fn description(&self) -> &str {
        self.description
    }
}
