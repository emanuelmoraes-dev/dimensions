pub trait TNpcRole {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
    fn description(&self) -> &str;
}
