use crate::item::Item;

pub enum Profession {
    Knight,
    Archer,
    Mage,
}

pub struct Character {
    pub name: String,
    pub profession: Profession,
    pub hp: i32,
    pub max_hp: i32,
    pub damage: i32,
    pub gold: i32,
    pub inventory: Vec<Item>,
}

impl Character {
    pub fn new(name: String, profession: Profession) -> Character {
        Character {
            name,
            profession,
            hp: 100,
            max_hp: 100,
            damage: 25,
            gold: 0,
            inventory: Vec::new(),
        }
    }
}
