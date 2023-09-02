use crate::assets::fonts::Fonts;

pub mod creator_config;
pub mod i18n;

#[derive(Clone)]
pub struct Config {
    pub fonts: Fonts,
}

impl Config {
    pub fn new() -> Self {
        Self {
            fonts: Fonts::new(),
        }
    }
}
