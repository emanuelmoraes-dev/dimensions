pub mod functional;

use crate::ports::traits::t_id::TId;
use crate::ports::traits::t_feedstock::TFeedstock;

use super::meta::Meta;
use super::universe::Universe;

#[macro_export]
macro_rules! fsdef {
    () => {
        Box<dyn Fn(&crate::act::meta::Meta, &crate::act::universe::Universe) -> u32>
    };
}

pub struct Feedstock<'m, T: TId> {
    value: T,
    meta: &'m Meta<'m>,
    def: fsdef![]
}

impl<'m, T: TId> Feedstock<'m, T> {
    pub fn new(value: T, meta: &'m Meta<'m>, def: fsdef![]) -> Self {
        Self { value, meta, def }
    }
}

impl<'m, ID: TId> TId for Feedstock<'m, ID> {
    fn id(&self) -> &str {
        self.value.id()
    }
}

impl<'m, T: TId> TFeedstock<T, Universe> for Feedstock<'m, T> {
    fn value(&self) -> &T {
        &self.value
    }
    fn probability<'u>(&self, universe: &'u Universe) -> u32 {
        let def = &self.def;
        def(self.meta, universe)
    }
}
