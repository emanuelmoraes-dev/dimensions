pub trait TOptId {
    fn id(&self) -> Option<&str>;
}

pub trait TId {
    fn id(&self) -> &str;
}

impl<ID: TId> TOptId for ID {
    fn id(&self) -> Option<&str> {
        Some(self.id())
    }
}
