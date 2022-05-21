use crate::ports::traits::role::TNpcRole;

use super::itens::Item;

pub enum SubjectAttrType {
    Vitality,
    Strength,
    Dexterity,
    Stamina,
    Weight,
    Speed,
    Intelligence,
    FlySpeed,
    FlyDuration,
    FlyAltitude,
    Swim,
    Breath,
    Endurance,
    ResistancePhysics,
    ResistanceMagic,
    ResistanceFire,
    ResistanceWater,
    ResistanceElectricity,
    ResistanceAir,
    Luck
}

pub struct SubjectAttr {
    pub atype: SubjectAttrType,
    pub title: &'static str,
    pub description: &'static str,
    pub points: i32,
    pub absorb: i32
}

pub struct Inventory {
    pub capacity: u32,
    pub itens: Vec<Item>
}

pub struct Subject {
    pub inventory: Inventory,
    pub attrs: [SubjectAttr; 20],
    pub hearts: u32
}

pub struct Player {
    pub subject: Subject,
    pub nickname: String,
    pub description: String
}

pub struct Npc<R: TNpcRole> {
    pub subject: Subject,
    pub roles: Vec<R>,
    pub name: String,
    pub description: String
}
