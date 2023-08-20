pub mod functional;

use crate::ports::traits::t_feedstock::TFeedstock;
use crate::ports::traits::t_id::TId;

use super::meta::Meta;
use super::universe::Universe;

#[macro_export]
macro_rules! fsdef {
    () => {
        Box<dyn Fn(usize, usize, &crate::dimpl::meta::Meta, &crate::dimpl::universe::Universe) -> u32>
    };
}

pub struct Feedstock<'m, T: TId> {
    pub require: Vec<String>,
    value: T,
    meta: &'m Meta<'m>,
    def: fsdef![],
}

impl<'m, T: TId> Feedstock<'m, T> {
    pub fn new(require: Vec<String>, value: T, meta: &'m Meta<'m>, def: fsdef![]) -> Self {
        Self {
            require,
            value,
            meta,
            def,
        }
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
    fn probability<'u>(&self, x: usize, y: usize, universe: &'u Universe) -> u32 {
        let def = &self.def;
        def(x, y, self.meta, universe)
    }
}
