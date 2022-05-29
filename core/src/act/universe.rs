use super::{map::Map, stock::Stock};

pub struct Universe {
    pub maps: Stock<Map>
}
