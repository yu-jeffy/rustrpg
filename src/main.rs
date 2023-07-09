mod character;
mod enemy;
mod game;

use character::Character;
use character::Profession;
use enemy::Enemy;
use game::Game;

fn main() {
    let player = Character::new(String::from("Rusty"), Profession::Knight);
    let enemies = vec![
        Enemy::new(String::from("Rust Goblin"), 30, 10),
        Enemy::new(String::from("Memoryleak Monster"), 60, 15),
        Enemy::new(String::from("Buggy Bug"), 75, 20),
    ];
    let mut game = Game::new(player, enemies);
    game.run();
}