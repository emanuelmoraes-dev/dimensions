use super::itens::Item;
use crate::ports::traits::t_role::TNpcRole;

pub enum SubjectAttrTypeEnum {
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
    Luck,
}

pub struct SubjectAttr {
    pub atype: SubjectAttrTypeEnum,
    pub title: &'static str,
    pub description: &'static str,
    pub points: i32,
    pub absorb: i32,
}

pub struct Inventory {
    pub capacity: u32,
    pub itens: Vec<Item>,
}

pub struct Subject {
    pub inventory: Inventory,
    pub attrs: [SubjectAttr; 20],
    pub hearts: u32,
}

pub struct Player {
    pub subject: Subject,
    pub nickname: String,
    pub description: String,
}

pub struct Npc {
    pub subject: Subject,
    pub roles: Vec<Box<dyn TNpcRole>>,
    pub name: String,
    pub description: String,
}
