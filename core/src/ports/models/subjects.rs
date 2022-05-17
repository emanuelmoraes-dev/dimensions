pub enum AttrType {
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

pub struct Attr {
    pub atype: AttrType,
    pub title: &'static str,
    pub description: &'static str,
    pub points: i64,
    pub absorb: i64
}

pub struct Subject {
    pub attrs: [Attr; 20]
}

pub struct Player {
    pub subject: Subject,
    pub nickname: String,
    pub description: String
}

pub struct NpcRole {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str
}

pub struct Npc {
    pub subject: Subject,
    pub roles: Vec<NpcRole>,
    pub name: String,
    pub description: String
}
