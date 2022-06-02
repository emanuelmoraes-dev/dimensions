use super::stock::Stock;
use super::dimension::Dimension;

pub struct Universe {
    pub maps: Stock<Dimension>,
}
