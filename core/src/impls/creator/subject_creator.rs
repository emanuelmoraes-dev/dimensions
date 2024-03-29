use crate::impls::config::creator_config::CreatorConfig;

use crate::impls::config::i18n::I18nSubjectAttr;
use crate::ports::models::m_subjects::{Subject, Inventory, SubjectAttrTypeEnum, SubjectAttr};

pub fn new_subject(config: &CreatorConfig) -> Subject {
    Subject {
        hearts: config.default.hearts,
        inventory: new_inventory(config),
        attrs: new_attrs(config),
    }
}

fn new_inventory(config: &CreatorConfig) -> Inventory {
    Inventory {
        capacity: config.default.inventory_capacity,
        itens: Vec::new(),
    }
}

fn new_attr(config: &CreatorConfig, atype: SubjectAttrTypeEnum, i18n: &I18nSubjectAttr) -> SubjectAttr {
    SubjectAttr {
        atype,
        title: i18n.title,
        description: i18n.description,
        points: config.default.points,
        absorb: config.default.absorb,
    }
}

fn new_attrs(config: &CreatorConfig) -> [SubjectAttr; 20] {
    let i18n = &config.i18n.subject_attrs;
    [
        new_attr(config, SubjectAttrTypeEnum::Vitality, &i18n.vitality),
        new_attr(config, SubjectAttrTypeEnum::Strength, &i18n.strength),
        new_attr(config, SubjectAttrTypeEnum::Dexterity, &i18n.dexterity),
        new_attr(config, SubjectAttrTypeEnum::Stamina, &i18n.stamina),
        new_attr(config, SubjectAttrTypeEnum::Weight, &i18n.weight),
        new_attr(config, SubjectAttrTypeEnum::Speed, &i18n.speed),
        new_attr(config, SubjectAttrTypeEnum::Intelligence, &i18n.intelligence),
        new_attr(config, SubjectAttrTypeEnum::FlySpeed, &i18n.fly_speed),
        new_attr(config, SubjectAttrTypeEnum::FlyDuration, &i18n.fly_duration),
        new_attr(config, SubjectAttrTypeEnum::FlyAltitude, &i18n.fly_altitude),
        new_attr(config, SubjectAttrTypeEnum::Swim, &i18n.swim),
        new_attr(config, SubjectAttrTypeEnum::Breath, &i18n.breath),
        new_attr(config, SubjectAttrTypeEnum::Endurance, &i18n.endurance),
        new_attr(
            config,
            SubjectAttrTypeEnum::ResistancePhysics,
            &i18n.resistance_physics,
        ),
        new_attr(
            config,
            SubjectAttrTypeEnum::ResistanceMagic,
            &i18n.resistance_magic,
        ),
        new_attr(
            config,
            SubjectAttrTypeEnum::ResistanceFire,
            &i18n.resistance_fire,
        ),
        new_attr(
            config,
            SubjectAttrTypeEnum::ResistanceWater,
            &i18n.resistance_water,
        ),
        new_attr(
            config,
            SubjectAttrTypeEnum::ResistanceElectricity,
            &i18n.resistance_electricity,
        ),
        new_attr(config, SubjectAttrTypeEnum::ResistanceAir, &i18n.resistance_air),
        new_attr(config, SubjectAttrTypeEnum::Luck, &i18n.luck),
    ]
}
