use crate::assets::fonts::FontsData;

pub mod creator_config;
pub mod i18n;

#[derive(Clone)]
pub struct Config {
    pub fonts: FontsData,
}

impl Config {
    pub fn new() -> Self {
        Self {
            fonts: FontsData::new(),
        }
    }
}
