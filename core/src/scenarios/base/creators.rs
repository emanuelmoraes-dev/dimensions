use crate::ports::models::subjects::{SubjectAttr, SubjectAttrType, Inventory, Subject, Player, Npc, NpcRole};

const DEFAULT_POINTS: i32 = 100;
const DEFAULT_ABSORB: i32 = 10;
const DEFAULT_HEARTS: u32 = 1000;
const DEFAULT_INVENTORY_CAPACITY: u32 = 50;

fn new_attr(atype: SubjectAttrType, title: &'static str, description: &'static str) -> SubjectAttr {
    SubjectAttr { atype, title, description, points: DEFAULT_POINTS, absorb: DEFAULT_ABSORB }
}

fn new_attrs() -> [SubjectAttr; 20] {
    [
        new_attr(
            SubjectAttrType::Vitality,
            "Vitality",
            "Vitality"),
        new_attr(
            SubjectAttrType::Strength,
            "Strength",
            "Strength"),
        new_attr(
            SubjectAttrType::Dexterity,
            "Dexterity",
            "Dexterity"),
        new_attr(
            SubjectAttrType::Stamina,
            "Stamina",
            "Stamina"),
        new_attr(
            SubjectAttrType::Weight,
            "Weight",
            "Weight"),
        new_attr(
            SubjectAttrType::Speed,
            "Speed",
            "Speed"),
        new_attr(
            SubjectAttrType::Intelligence,
            "Intelligence",
            "Intelligence"),
        new_attr(SubjectAttrType::FlySpeed,
            "Fly (speed)",
            "Fly (speed)"),
        new_attr(SubjectAttrType::FlyDuration,
            "Fly (duration)",
            "Fly (duration)"),
        new_attr(SubjectAttrType::FlyAltitude,
            "Fly (altitude)",
            "Fly (altitude)"),
        new_attr(
            SubjectAttrType::Swim,
            "Swim",
            "Swim"),
        new_attr(
            SubjectAttrType::Breath,
            "Breath",
            "Breath"),
        new_attr(
            SubjectAttrType::Endurance,
            "Endurance",
            "Endurance"),
        new_attr(
            SubjectAttrType::ResistancePhysics,
            "Resistance (physics)",
            "Resistance (physics)"),
        new_attr(
            SubjectAttrType::ResistanceMagic,
            "Resistance (magic)",
            "Resistance (magic)"),
        new_attr(
            SubjectAttrType::ResistanceFire,
            "Resistance (fire)",
            "Resistance (fire)"),
        new_attr(
            SubjectAttrType::ResistanceWater,
            "Resistance (water)",
            "Resistance (water)"),
        new_attr(
            SubjectAttrType::ResistanceElectricity,
            "Resistance (electricity)",
            "Resistance (electricity)"),
        new_attr(
            SubjectAttrType::ResistanceAir,
            "Resistance (air)",
            "Resistance (air)"),
        new_attr(
            SubjectAttrType::Luck,
            "Luck",
            "Luck")
    ]
}

fn new_inventory() -> Inventory {
    Inventory { capacity: DEFAULT_INVENTORY_CAPACITY, itens: Vec::new() }
}

fn new_subject() -> Subject {
    Subject { inventory: new_inventory(), attrs: new_attrs() }
}

pub trait PlayerCreator {
    fn new(nickname: String, description: String) -> Player;
}

impl PlayerCreator for Player {
    fn new(nickname: String, description: String) -> Player {
        Player {
            subject: new_subject(),
            nickname,
            description,
            hearts: DEFAULT_HEARTS
        }
    }
}

pub trait NpcCreator<R: NpcRole> {
    fn new(name: String, description: String, roles: Vec<R>) -> Npc<R>;
}

impl<R: NpcRole> NpcCreator<R> for Npc<R> {
    fn new(name: String, description: String, roles: Vec<R>) -> Npc<R> {
        Npc {
            subject: new_subject(),
            roles,
            name,
            description
        }
    }
}
