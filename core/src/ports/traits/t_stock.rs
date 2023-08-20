use super::t_id::TOptId;

pub trait TStock<T: TOptId> {
    fn itens(&self) -> &[T];
    fn add(&mut self, item: T);
    fn remove(&mut self, id: &str) -> Option<T>;
}
