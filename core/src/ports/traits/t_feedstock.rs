use super::t_id::TId;

pub trait TFeedstock<T: TId, U> {
    fn value(&self) -> &T;
    fn probability<'u>(&self, universe: &'u U) -> u32;
}
