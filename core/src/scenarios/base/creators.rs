use crate::ports::models::subjects::{Attr, AttrType, Subject, Player, Npc, NpcRole};

const DEFAULT_POINTS: i64 = 100;
const DEFAULT_ABSORB: i64 = 10;

fn new_attr(atype: AttrType, title: &'static str, description: &'static str) -> Attr {
    Attr { atype, title, description, points: DEFAULT_POINTS, absorb: DEFAULT_ABSORB }
}

fn new_subject() -> Subject {
    Subject { attrs: [
        new_attr(
            AttrType::Vitality,
            "Vitality",
            "Vitality"),
        new_attr(
            AttrType::Strength,
            "Strength",
            "Strength"),
        new_attr(
            AttrType::Dexterity,
            "Dexterity",
            "Dexterity"),
        new_attr(
            AttrType::Stamina,
            "Stamina",
            "Stamina"),
        new_attr(
            AttrType::Weight,
            "Weight",
            "Weight"),
        new_attr(
            AttrType::Speed,
            "Speed",
            "Speed"),
        new_attr(
            AttrType::Intelligence,
            "Intelligence",
            "Intelligence"),
        new_attr(AttrType::FlySpeed,
            "Fly (speed)",
            "Fly (speed)"),
        new_attr(AttrType::FlyDuration,
            "Fly (duration)",
            "Fly (duration)"),
        new_attr(AttrType::FlyAltitude,
            "Fly (altitude)",
            "Fly (altitude)"),
        new_attr(
            AttrType::Swim,
            "Swim",
            "Swim"),
        new_attr(
            AttrType::Breath,
            "Breath",
            "Breath"),
        new_attr(
            AttrType::Endurance,
            "Endurance",
            "Endurance"),
        new_attr(
            AttrType::ResistancePhysics,
            "Resistance (physics)",
            "Resistance (physics)"),
        new_attr(
            AttrType::ResistanceMagic,
            "Resistance (magic)",
            "Resistance (magic)"),
        new_attr(
            AttrType::ResistanceFire,
            "Resistance (fire)",
            "Resistance (fire)"),
        new_attr(
            AttrType::ResistanceWater,
            "Resistance (water)",
            "Resistance (water)"),
        new_attr(
            AttrType::ResistanceElectricity,
            "Resistance (electricity)",
            "Resistance (electricity)"),
        new_attr(
            AttrType::ResistanceAir,
            "Resistance (air)",
            "Resistance (air)"),
        new_attr(
            AttrType::Luck,
            "Luck",
            "Luck")
    ]}
}

pub trait PlayerCreator {
    fn new(nickname: String, description: String) -> Player;
}

impl PlayerCreator for Player {
    fn new(nickname: String, description: String) -> Player {
        Player {
            subject: new_subject(),
            nickname,
            description
        }
    }
}

pub trait NpcCreator {
    fn new(name: String, description: String, roles: Vec<NpcRole>) -> Npc;
}

impl NpcCreator for Npc {
    fn new(name: String, description: String, roles: Vec<NpcRole>) -> Npc {
        Npc {
            subject: new_subject(),
            roles,
            name,
            description
        }
    }
}
