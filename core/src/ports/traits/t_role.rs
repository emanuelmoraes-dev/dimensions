use super::t_id::TId;

pub trait TNpcRole: TId {
    fn title(&self) -> &str;
    fn description(&self) -> &str;
}
