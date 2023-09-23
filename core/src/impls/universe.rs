use image::RgbaImage;

use crate::ports::models::m_subjects::Player;
use crate::ports::traits::t_universe::TUniverse;

use super::config::Config;
use super::map::Map;
use super::stock::Stock;

pub struct Universe {
    pub config: Config,
    pub player: Player,
    pub maps: Stock<Map>,
    pub current_map: Map,
}

impl Universe {
    pub fn new(config: Config, player: Player) -> Self {
        let maps: Stock<Map> = Stock::new();
        let current_map: Map = Map::new(None);
        Self {
            config,
            player,
            maps,
            current_map,
        }
    }
}

impl TUniverse<RgbaImage, Map> for Universe {
    fn current_map(&mut self) -> &mut Map {
        &mut self.current_map
    }
}
