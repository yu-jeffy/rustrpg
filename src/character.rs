pub enum Profession {
    Knight,
    Archer,
    Mage,
}

pub struct Character {
    pub name: String,
    pub profession: Profession,
    pub hp: i32,
    pub damage: i32,
    pub gold: i32,
}

impl Character {
    pub fn new(name: String, profession: Profession) -> Character {
        Character {
            name,
            profession,
            hp: 100,
            damage: 25,
            gold: 0,
        }
    }
}
