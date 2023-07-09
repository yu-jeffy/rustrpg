#[derive(Clone, Debug)]
pub struct Item {
    pub name: String,
    pub healing_value: i32,
    pub cost: i32,
}

impl Item {
    pub fn new(name: String, healing_value: i32, cost: i32) -> Item {
        Item {
            name,
            healing_value,
            cost,
        }
    }
}