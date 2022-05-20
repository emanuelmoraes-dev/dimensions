pub struct I18nSubjectAttr {
    pub title: &'static str,
    pub description: &'static str
}

impl I18nSubjectAttr {
    fn new(title: &'static str, description: &'static str) -> I18nSubjectAttr {
        I18nSubjectAttr { title, description }
    }
}

pub struct I18nSubjectAttrs {
    pub vitality: I18nSubjectAttr,
    pub strength: I18nSubjectAttr,
    pub dexterity: I18nSubjectAttr,
    pub stamina: I18nSubjectAttr,
    pub weight: I18nSubjectAttr,
    pub speed: I18nSubjectAttr,
    pub intelligence: I18nSubjectAttr,
    pub fly_speed: I18nSubjectAttr,
    pub fly_duration: I18nSubjectAttr,
    pub fly_altitude: I18nSubjectAttr,
    pub swim: I18nSubjectAttr,
    pub breath: I18nSubjectAttr,
    pub endurance: I18nSubjectAttr,
    pub resistance_physics: I18nSubjectAttr,
    pub resistance_magic: I18nSubjectAttr,
    pub resistance_fire: I18nSubjectAttr,
    pub resistance_water: I18nSubjectAttr,
    pub resistance_electricity: I18nSubjectAttr,
    pub resistance_air: I18nSubjectAttr,
    pub luck: I18nSubjectAttr
}

pub struct I18n {
    pub subject_attrs: I18nSubjectAttrs
}

pub fn default_i18n() -> I18n {
    I18n { subject_attrs: I18nSubjectAttrs {
        vitality: I18nSubjectAttr::new(
            "Vitality",
            "Vitality"),
        strength: I18nSubjectAttr::new(
            "Strength",
            "Strength"),
        dexterity: I18nSubjectAttr::new(
            "Dexterity",
            "Dexterity"),
        stamina: I18nSubjectAttr::new(
            "Stamina",
            "Stamina"),
        weight: I18nSubjectAttr::new(
            "Weight",
            "Weight"),
        speed: I18nSubjectAttr::new(
            "Speed",
            "Speed"),
        intelligence: I18nSubjectAttr::new(
            "Intelligence",
            "Intelligence"),
        fly_speed: I18nSubjectAttr::new(
            "Fly (speed)",
            "Fly (speed)"),
        fly_duration: I18nSubjectAttr::new(
            "Fly (duration)",
            "Fly (duration)"),
        fly_altitude: I18nSubjectAttr::new(
            "Fly (altitude)",
            "Fly (altitude)"),
        swim: I18nSubjectAttr::new(
            "Swim",
            "Swim"),
        breath: I18nSubjectAttr::new(
            "Breath",
            "Breath"),
        endurance: I18nSubjectAttr::new(
            "Endurance",
            "Endurance"),
        resistance_physics: I18nSubjectAttr::new(
            "Resistance (physics)",
            "Resistance (physics)"),
        resistance_magic: I18nSubjectAttr::new(
            "Resistance (magic)",
            "Resistance (magic)"),
        resistance_fire: I18nSubjectAttr::new(
            "Resistance (fire)",
            "Resistance (fire)"),
        resistance_water: I18nSubjectAttr::new(
            "Resistance (water)",
            "Resistance (water)"),
        resistance_electricity: I18nSubjectAttr::new(
            "Resistance (electricity)",
            "Resistance (electricity)"),
        resistance_air: I18nSubjectAttr::new(
            "Resistance (air)",
            "Resistance (air)"),
        luck: I18nSubjectAttr::new(
            "Luck",
            "Luck")
    }}
}