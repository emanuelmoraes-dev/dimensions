pub struct I18nSubjectAttr {
    pub title: &'static str,
    pub description: &'static str
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

impl I18nSubjectAttrs {
    fn new() -> Self {
        Self {
            vitality: I18nSubjectAttr{
                title: "Vitality",
                description: "Vitality"},
            strength: I18nSubjectAttr{
                title: "Strength",
                description: "Strength"},
            dexterity: I18nSubjectAttr{
                title: "Dexterity",
                description: "Dexterity"},
            stamina: I18nSubjectAttr{
                title: "Stamina",
                description: "Stamina"},
            weight: I18nSubjectAttr{
                title: "Weight",
                description: "Weight"},
            speed: I18nSubjectAttr{
                title: "Speed",
                description: "Speed"},
            intelligence: I18nSubjectAttr{
                title: "Intelligence",
                description: "Intelligence"},
            fly_speed: I18nSubjectAttr{
                title: "Fly (speed)",
                description: "Fly (speed)"},
            fly_duration: I18nSubjectAttr{
                title: "Fly (duration)",
                description: "Fly (duration)"},
            fly_altitude: I18nSubjectAttr{
                title: "Fly (altitude)",
                description: "Fly (altitude)"},
            swim: I18nSubjectAttr{
                title: "Swim",
                description: "Swim"},
            breath: I18nSubjectAttr{
                title: "Breath",
                description: "Breath"},
            endurance: I18nSubjectAttr{
                title: "Endurance",
                description: "Endurance"},
            resistance_physics: I18nSubjectAttr{
                title: "Resistance (physics)",
                description: "Resistance (physics)"},
            resistance_magic: I18nSubjectAttr{
                title: "Resistance (magic)",
                description: "Resistance (magic)"},
            resistance_fire: I18nSubjectAttr{
                title: "Resistance (fire)",
                description: "Resistance (fire)"},
            resistance_water: I18nSubjectAttr{
                title: "Resistance (water)",
                description: "Resistance (water)"},
            resistance_electricity: I18nSubjectAttr{
                title: "Resistance (electricity)",
                description: "Resistance (electricity)"},
            resistance_air: I18nSubjectAttr{
                title: "Resistance (air)",
                description: "Resistance (air)"},
            luck: I18nSubjectAttr{
                title: "Luck",
                description: "Luck"}
        }
    }
}

pub struct I18n {
    pub subject_attrs: I18nSubjectAttrs
}

impl I18n {
    pub fn new() -> Self {
        Self { subject_attrs: I18nSubjectAttrs::new() }
    }
}
