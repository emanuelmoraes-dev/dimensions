use std::collections::HashMap;

use crate::ports::traits::t_feedstock::TFeedstock;
use crate::ports::traits::t_id::TId;
use crate::ports::traits::t_stock::TStock;

use super::feedstock::Feedstock;

pub struct Stock<'a, T: TId> {
    itens: Vec<Feedstock<'a, T>>,
    map: HashMap<String, usize>
}

impl<'a, T: TId> Stock<'a, T> {
    pub fn new(itens: Vec<Feedstock<'a, T>>) -> Self {
        let mut instance = Self { itens, map: HashMap::new() };
        instance.build_map(None);
        instance
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

impl<'a, T: TId> TStock<'a, T, Feedstock<'a, T>> for Stock<'a, T> {
    fn itens(&self) -> &[Feedstock<'a, T>] {
        &self.itens[..]
    }
    fn add(&mut self, item: Feedstock<'a, T>) {
        let id = item.value().id().to_string();
        let index = self.itens.len();
        self.itens.push(item);
        self.build_map(Some((index, id)));
    }
    fn map(&self) -> &HashMap<String, usize> {
        &self.map
    }
}
