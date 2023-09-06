use super::i18n::I18n;

pub struct DefaultConfigValues {
    pub points: i32,
    pub absorb: i32,
    pub hearts: u32,
    pub inventory_capacity: u32,
}

impl DefaultConfigValues {
    pub fn new() -> Self {
        Self {
            points: 100,
            absorb: 10,
            hearts: 1000,
            inventory_capacity: 50,
        }
    }
}

pub struct CreatorConfig {
    pub default: DefaultConfigValues,
    pub i18n: I18n,
}

impl CreatorConfig {
    pub fn new() -> Self {
        Self {
            default: DefaultConfigValues::new(),
            i18n: I18n::new(),
        }
    }
}
