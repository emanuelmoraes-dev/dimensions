use std::collections::HashMap;

use super::t_id::TId;
use super::t_feedstock::TFeedstock;

pub trait TStock<'a, T: TId, FS: TFeedstock<T>> {
    fn itens(&self) -> &[FS];
    fn add(&mut self, item: FS);
    fn map(&self) -> &HashMap<String, usize>;
}
