use super::t_id::TId;

pub trait TFeedstock<T: TId> {
    fn value(&self) -> &T;
    fn probability(&self) -> u32;
}
