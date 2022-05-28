use crate::ports::traits::{t_id::TId, t_feedstock::TFeedstock};

use super::stock::Stock;

pub struct Feedstock<'s, T: TId> {
    value: T,
    stock: &'s Stock<'s, T>
}

impl<'s, T: TId> Feedstock<'s, T> {
    pub fn new(value: T, stock: &'s Stock<'s, T>) -> Self {
        Self { value, stock }
    }
}

impl<'s, T: TId> TFeedstock<T> for Feedstock<'s, T> {
    fn value(&self) -> &T {
        &self.value
    }
    fn probability(&self) -> u32 {
        0 // TODO
    }
}
