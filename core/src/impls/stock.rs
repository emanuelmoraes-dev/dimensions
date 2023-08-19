use std::collections::HashMap;

use crate::ports::traits::t_id::TOptId;
use crate::ports::traits::t_stock::TStock;

pub struct Stock<T: TOptId> {
    itens: Vec<T>,
    map: HashMap<String, usize>,
}

impl<T: TOptId> Stock<T> {
    pub fn new() -> Self {
        Self {
            itens: Vec::new(),
            map: HashMap::new(),
        }
    }
    pub fn set_values(&mut self, itens: Vec<T>) {
        self.itens = itens;
        self.build_map(None);
    }
    fn build_map(&mut self, item: Option<(usize, Option<String>)>) {
        if let Some((index, id)) = item {
            if let Some(id) = id {
                self.map.insert(id, index);
            }
        } else {
            for (i, item) in self.itens.iter().enumerate() {
                if let Some(id) = item.id() {
                    self.map.insert(id.to_string(), i);
                }
            }
        }
    }
}

impl<T: TOptId> TStock<T> for Stock<T> {
    fn itens(&self) -> &[T] {
        &self.itens[..]
    }
    fn add(&mut self, item: T) {
        let id = item.id().and_then(|id| Some(id.to_string()));
        let index = self.itens.len();
        self.itens.push(item);
        self.build_map(Some((index, id)));
    }
    fn map(&self) -> &HashMap<String, usize> {
        &self.map
    }
}
