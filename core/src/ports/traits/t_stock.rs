use std::collections::HashMap;

use super::t_id::TId;
use super::t_feedstock::TFeedstock;

pub trait TStock<T: TId, U, F: TFeedstock<T, U>> {
    fn itens(&self) -> &[F];
    fn add(&mut self, item: F);
    fn map(&self) -> &HashMap<String, usize>;
}
