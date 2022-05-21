pub trait TNpcRole {
    fn get_id(&self) -> &str;
    fn get_title(&self) -> &str;
    fn get_description(&self) -> &str;
}
