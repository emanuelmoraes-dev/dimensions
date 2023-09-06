use wasm_bindgen::prelude::*;

use crate::d_impl::config::Config;
use crate::ports::traits::t_creator::TCreator;

use crate::d_impl::universe::Universe;
use crate::d_impl::creator::Creator;
use crate::d_impl::config::creator_config::CreatorConfig;
use crate::d_impl::meta::Meta;

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
