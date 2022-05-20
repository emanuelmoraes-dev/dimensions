use crate::ports::models::subjects::{SubjectAttr, SubjectAttrType, Inventory, Subject, Player, Npc, NpcRole};
use super::super::config::i18n::{I18nSubjectAttr, default_i18n};

const DEFAULT_POINTS: i32 = 100;
const DEFAULT_ABSORB: i32 = 10;
const DEFAULT_HEARTS: u32 = 1000;
const DEFAULT_INVENTORY_CAPACITY: u32 = 50;

fn new_attr(atype: SubjectAttrType, i18n: I18nSubjectAttr) -> SubjectAttr {
    SubjectAttr { atype, title: i18n.title, description: i18n.description, points: DEFAULT_POINTS, absorb: DEFAULT_ABSORB }
}

fn new_attrs() -> [SubjectAttr; 20] {
    let i18n = default_i18n().subject_attrs;
    [
        new_attr(SubjectAttrType::Vitality, i18n.vitality),
        new_attr(SubjectAttrType::Strength, i18n.strength),
        new_attr(SubjectAttrType::Dexterity, i18n.dexterity),
        new_attr(SubjectAttrType::Stamina, i18n.stamina),
        new_attr(SubjectAttrType::Weight, i18n.weight),
        new_attr(SubjectAttrType::Speed, i18n.speed),
        new_attr(SubjectAttrType::Intelligence, i18n.intelligence),
        new_attr(SubjectAttrType::FlySpeed, i18n.fly_speed),
        new_attr(SubjectAttrType::FlyDuration, i18n.fly_duration),
        new_attr(SubjectAttrType::FlyAltitude, i18n.fly_altitude),
        new_attr(SubjectAttrType::Swim, i18n.swim),
        new_attr(SubjectAttrType::Breath, i18n.breath),
        new_attr(SubjectAttrType::Endurance, i18n.endurance),
        new_attr(SubjectAttrType::ResistancePhysics, i18n.resistance_physics),
        new_attr(SubjectAttrType::ResistanceMagic, i18n.resistance_magic),
        new_attr(SubjectAttrType::ResistanceFire, i18n.resistance_fire),
        new_attr(SubjectAttrType::ResistanceWater, i18n.resistance_water),
        new_attr(SubjectAttrType::ResistanceElectricity, i18n.resistance_electricity),
        new_attr(SubjectAttrType::ResistanceAir, i18n.resistance_air),
        new_attr(SubjectAttrType::Luck, i18n.luck)
    ]
}

fn new_inventory() -> Inventory {
    Inventory { capacity: DEFAULT_INVENTORY_CAPACITY, itens: Vec::new() }
}

fn new_subject() -> Subject {
    Subject { inventory: new_inventory(), attrs: new_attrs() }
}

pub trait PlayerCreator {
    fn create_player(nickname: String, description: String) -> Player;
}

impl PlayerCreator for Player {
    fn create_player(nickname: String, description: String) -> Player {
        Player {
            subject: new_subject(),
            nickname,
            description,
            hearts: DEFAULT_HEARTS
        }
    }
}

pub trait NpcCreator<R: NpcRole> {
    fn create_npc(name: String, description: String, roles: Vec<R>) -> Npc<R>;
}

impl<R: NpcRole> NpcCreator<R> for Npc<R> {
    fn create_npc(name: String, description: String, roles: Vec<R>) -> Npc<R> {
        Npc {
            subject: new_subject(),
            roles,
            name,
            description
        }
    }
}
