use crate::ports::traits::t_id::TId;

pub enum ItemAttrTypeEnum {
    Strength,
    Dexterity,
    Weight,
    Intelligence,
    FlySpeed,
    FlyDuration,
    FlyAltitude,
    Swim,
    Breath,
    Endurance,
    Luck,
}

pub struct ItemAttr {
    pub atype: ItemAttrTypeEnum,
    pub title: &'static str,
    pub description: &'static str,
    pub damage: i32,
    pub requirement: i32,
}

pub struct Item {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub attrs: Vec<ItemAttr>,
}

impl TId for Item {
    fn id(&self) -> &str {
        self.id
    }
}
