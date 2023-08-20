use super::config::Config;
use super::dimension::Dimension;
use super::stock::Stock;

pub struct Universe {
    pub config: Config,
    pub maps: Stock<Dimension>,
}
