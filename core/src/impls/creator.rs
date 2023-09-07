use image::RgbaImage;

use self::subject_creator::new_subject;

use super::config::Config;
use super::config::creator_config::CreatorConfig;
use super::map::Map;
use super::gen::Gen;
use super::meta::Meta;
use super::universe::Universe;

use crate::ports::models::m_location::{Location, LocationTypeEnum};
use crate::ports::models::m_style::{TextStyle, AlignEnum};
use crate::ports::models::m_subjects::{Npc, Player};
use crate::ports::traits::t_creator::TCreator;
use crate::ports::traits::t_gen::{TGen, TImageGen};
use crate::ports::traits::t_id::TId;
use crate::ports::traits::t_role::TNpcRole;

mod subject_creator;

fn get_random_location_type() -> LocationTypeEnum {
    let types = vec![
        LocationTypeEnum::Ground,
        LocationTypeEnum::Cave,
        LocationTypeEnum::Wall,
        LocationTypeEnum::Gram,
        LocationTypeEnum::River,
    ];
    let index = rand::random::<usize>() % types.len();
    types[index].clone()
}

pub struct Creator {
    pub config: CreatorConfig,
    pub meta: Meta,
    pub gen: Gen
}

impl Creator {
    pub fn new(config: CreatorConfig, meta: Meta) -> Self {
        let gen: Gen = Gen::new();
        Self { config, meta, gen }
    }
    pub fn create_universe(&self, config: Config, player: Player) -> Universe {
        Universe::new(config, player)
    }
}

impl TCreator<RgbaImage, Map> for Creator {
    fn create_player(&self, nickname: String, description: String) -> Player {
        Player {
            subject: new_subject(&self.config),
            nickname,
            description,
        }
    }
    fn create_location(&self, _x: i32, _y: i32, width: u32, height: u32) -> Location<RgbaImage> {
        let ltype = get_random_location_type();
        let color = ltype.color();
        let text_style = TextStyle {
            font: self.config.fonts.roboto_bold,
            color: [0, 0, 0, 255],
            font_size: 20.0,
            align_x: AlignEnum::Center,
            align_y: AlignEnum::Center,
            offset_x: 0.0,
            offset_y: 0.0
        };
        let image = self.gen.image().text(&color, width, height, text_style, ltype.id());
        Location::new(ltype, image, None)
    }
    fn create_map(&self) -> Map {
        todo!("create_map")
    }
    fn create_npc(&self, name: String, description: String, roles: Vec<Box<dyn TNpcRole>>) -> Npc {
        Npc {
            subject: new_subject(&self.config),
            roles,
            name,
            description,
        }
    }
}
