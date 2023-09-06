use crate::ports::models::subjects::Player;
use crate::ports::traits::t_creator::TCreator;

use super::config::Config;
use super::creator::Creator;
use super::dimension::Dimension;
use super::gen::Gen;
use super::meta::Meta;
use super::stock::Stock;

pub struct Universe {
    pub config: Config,
    pub creator: Creator,
    pub meta: Meta,
    pub gen: Gen,
    pub player: Player,
    pub maps: Stock<Dimension>,
    pub current_map: Option<Dimension>,
}

impl Universe {
    pub fn new(nickname: String, description: String) -> Self {
        let config = Config::new();
        let creator = Creator::new();
        let meta = Meta::new();
        let gen = Gen::new();
        let player = creator.create_player(nickname, description);
        let maps: Stock<Dimension> = Stock::new();
        let current_map: Option<Dimension> = None;
        Self {
            config,
            creator,
            meta,
            gen,
            player,
            maps,
            current_map,
        }
    }
}
