pub struct Enemy {
    pub name: String,
    pub hp: i32,
    pub damage: i32,
}

impl Enemy {
    pub fn new(name: String, hp: i32, damage: i32) -> Enemy {
        Enemy {
            name,
            hp,
            damage,
        }
    }
}