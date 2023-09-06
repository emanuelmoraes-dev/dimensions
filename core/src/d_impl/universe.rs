use crate::ports::models::subjects::Player;

use super::config::Config;
use super::dimension::Dimension;
use super::gen::Gen;
use super::stock::Stock;

pub struct Universe {
    pub config: Config,
    pub gen: Gen,
    pub player: Player,
    pub maps: Stock<Dimension>,
    pub current_map: Option<Dimension>,
}

impl Universe {
    pub fn new(config: Config, player: Player) -> Self {
        let gen = Gen::new();
        let maps: Stock<Dimension> = Stock::new();
        let current_map: Option<Dimension> = None;
        Self {
            config,
            gen,
            player,
            maps,
            current_map,
        }
    }
}
