use super::config::i18n::I18nSubjectAttr;
use super::config::Config;
use crate::ports::models::subjects::{
    Inventory, Npc, Player, Subject, SubjectAttr, SubjectAttrType,
};
use crate::ports::traits::t_creator::TCreator;
use crate::ports::traits::t_role::TNpcRole;

pub struct Creator {
    pub config: Config,
}

impl Creator {
    pub fn new() -> Self {
        Self {
            config: Config::new(),
        }
    }
}

impl TCreator for Creator {
    fn create_player(&self, nickname: String, description: String) -> Player {
        Player {
            subject: new_subject(&self.config),
            nickname,
            description,
        }
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

fn new_subject(config: &Config) -> Subject {
    Subject {
        hearts: config.default.hearts,
        inventory: new_inventory(config),
        attrs: new_attrs(config),
    }
}

fn new_inventory(config: &Config) -> Inventory {
    Inventory {
        capacity: config.default.inventory_capacity,
        itens: Vec::new(),
    }
}

fn new_attr(config: &Config, atype: SubjectAttrType, i18n: &I18nSubjectAttr) -> SubjectAttr {
    SubjectAttr {
        atype,
        title: i18n.title,
        description: i18n.description,
        points: config.default.points,
        absorb: config.default.absorb,
    }
}

fn new_attrs(config: &Config) -> [SubjectAttr; 20] {
    let i18n = &config.i18n.subject_attrs;
    [
        new_attr(config, SubjectAttrType::Vitality, &i18n.vitality),
        new_attr(config, SubjectAttrType::Strength, &i18n.strength),
        new_attr(config, SubjectAttrType::Dexterity, &i18n.dexterity),
        new_attr(config, SubjectAttrType::Stamina, &i18n.stamina),
        new_attr(config, SubjectAttrType::Weight, &i18n.weight),
        new_attr(config, SubjectAttrType::Speed, &i18n.speed),
        new_attr(config, SubjectAttrType::Intelligence, &i18n.intelligence),
        new_attr(config, SubjectAttrType::FlySpeed, &i18n.fly_speed),
        new_attr(config, SubjectAttrType::FlyDuration, &i18n.fly_duration),
        new_attr(config, SubjectAttrType::FlyAltitude, &i18n.fly_altitude),
        new_attr(config, SubjectAttrType::Swim, &i18n.swim),
        new_attr(config, SubjectAttrType::Breath, &i18n.breath),
        new_attr(config, SubjectAttrType::Endurance, &i18n.endurance),
        new_attr(
            config,
            SubjectAttrType::ResistancePhysics,
            &i18n.resistance_physics,
        ),
        new_attr(
            config,
            SubjectAttrType::ResistanceMagic,
            &i18n.resistance_magic,
        ),
        new_attr(
            config,
            SubjectAttrType::ResistanceFire,
            &i18n.resistance_fire,
        ),
        new_attr(
            config,
            SubjectAttrType::ResistanceWater,
            &i18n.resistance_water,
        ),
        new_attr(
            config,
            SubjectAttrType::ResistanceElectricity,
            &i18n.resistance_electricity,
        ),
        new_attr(config, SubjectAttrType::ResistanceAir, &i18n.resistance_air),
        new_attr(config, SubjectAttrType::Luck, &i18n.luck),
    ]
}
