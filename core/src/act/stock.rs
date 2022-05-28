use std::collections::HashMap;

use crate::ports::traits::t_feedstock::TFeedstock;
use crate::ports::traits::t_id::TId;
use crate::ports::traits::t_stock::TStock;

use super::feedstock::Feedstock;
use super::universe::Universe;

pub struct Stock<'m, T: TId> {
    itens: Vec<Feedstock<'m, T>>,
    map: HashMap<String, usize>
}

impl<'m, T: TId> Stock<'m, T> {
    pub fn new() -> Self {
        Self { itens: Vec::new(), map: HashMap::new() }
    }
    pub fn set_values(&mut self, itens: Vec<Feedstock<'m, T>>) {
        self.itens = itens;
        self.build_map(None);
    }
    fn build_map(&mut self, item: Option<(usize, String)>) {
        if let Some((index, id)) = item {
            self.map.insert(id, index);
        } else {
            for (i, item) in self.itens.iter().enumerate() {
                let id = item.value().id().to_string();
                self.map.insert(id, i);
            }
        }
    }
}

impl<'m, T: TId> TStock<T, Universe, Feedstock<'m, T>> for Stock<'m, T> {
    fn itens(&self) -> &[Feedstock<'m, T>] {
        &self.itens[..]
    }
    fn add(&mut self, item: Feedstock<'m, T>) {
        let id = item.value().id().to_string();
        let index = self.itens.len();
        self.itens.push(item);
        self.build_map(Some((index, id)));
    }
    fn map(&self) -> &HashMap<String, usize> {
        &self.map
    }
}
