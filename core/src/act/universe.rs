use super::dimension::Dimension;
use super::stock::Stock;

pub struct Universe {
    pub maps: Stock<Dimension>
}
