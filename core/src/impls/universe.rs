use crate::ports::models::m_subjects::Player;

use super::config::Config;
use super::dimension::Dimension;
use super::stock::Stock;

pub struct Universe {
    pub config: Config,
    pub player: Player,
    pub maps: Stock<Dimension>,
    pub current_map: Dimension,
}

impl Universe {
    pub fn new(config: Config, player: Player) -> Self {
        let maps: Stock<Dimension> = Stock::new();
        let current_map: Dimension = Dimension::new(None);
        Self {
            config,
            player,
            maps,
            current_map,
        }
    }
}
