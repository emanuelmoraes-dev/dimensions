use std::collections::HashMap;

use super::t_id::TOptId;

pub trait TStock<T: TOptId> {
    fn itens(&self) -> &[T];
    fn add(&mut self, item: T);
    fn map(&self) -> &HashMap<String, usize>;
}
