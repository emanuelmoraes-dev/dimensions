pub mod functional;

use crate::ports::traits::t_feedstock::TFeedstock;
use crate::ports::traits::t_id::TId;

use super::universe::Universe;

#[macro_export]
macro_rules! fsdef {
    () => {
        Box<dyn Fn(usize, usize, &crate::d_impl::universe::Universe) -> u32>
    };
}

pub struct Feedstock<T: TId> {
    pub require: Vec<String>,
    value: T,
    def: fsdef![],
}

impl<T: TId> Feedstock<T> {
    pub fn new(require: Vec<String>, value: T, def: fsdef![]) -> Self {
        Self {
            require,
            value,
            def,
        }
    }
}

impl<ID: TId> TId for Feedstock<ID> {
    fn id(&self) -> &str {
        self.value.id()
    }
}

impl<T: TId> TFeedstock<T, Universe> for Feedstock<T> {
    fn value(&self) -> &T {
        &self.value
    }
    fn probability<'m, 'u>(&self, x: usize, y: usize, universe: &'u Universe) -> u32 {
        let def = &self.def;
        def(x, y, universe)
    }
}
