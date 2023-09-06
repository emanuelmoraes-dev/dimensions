use wasm_bindgen::prelude::*;

use crate::impls::config::Config;
use crate::ports::traits::t_creator::TCreator;

use crate::impls::universe::Universe;
use crate::impls::creator::Creator;
use crate::impls::config::creator_config::CreatorConfig;
use crate::impls::meta::Meta;

#[wasm_bindgen]
pub struct XCore {
    #[wasm_bindgen(skip)]
    pub creator: Creator,

    #[wasm_bindgen(skip)]
    pub universe: Universe
}

#[wasm_bindgen]
impl XCore {
    #[wasm_bindgen(constructor)]
    pub fn new(nickname: String, description: String) -> Self {
        let config = Config::new();
        let creator_config = CreatorConfig::new();

        let meta = Meta::new();
        let creator = Creator::new(creator_config, meta);
        let player = creator.create_player(nickname, description);
        let universe = creator.create_universe(config, player);
        Self { creator, universe }
    }
}
