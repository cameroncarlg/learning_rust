#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub role: String,
    pub mana_user: bool,
    pub max_hp: i32,
    pub abilities: Vec<String>,
}

pub struct Monster {
    pub type_of_monster: String,
    pub max_attack: i32,
    pub max_hp: i32,
    pub skin_color: String,
}

pub struct Inventory {
    pub max_size: i32,
    pub material_type: String,
}

pub trait Describe {
    fn describe(&self) -> String;

    fn describe_unit(&self) -> String {
        format!("Before you lies a {}...", self.describe())
    }
}

impl Describe for Class {
    fn describe(&self) -> String {
        format!("{}, with a role of {}", self.name, self.role)
    }
}

impl Describe for Monster {
    fn describe(&self) -> String {
        format!("{}, with {} skin", self.type_of_monster, self.skin_color)
    }
}
