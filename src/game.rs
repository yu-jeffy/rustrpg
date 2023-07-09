use std::io;
use std::io::prelude::*;
use crate::character::Character;
use crate::enemy::Enemy;
use crate::item::Item;

pub struct Game {
    pub player: Character,
    pub enemies: Vec<Enemy>,
    pub stage: usize,
}

impl Game {
    pub fn new(player: Character, enemies: Vec<Enemy>) -> Game {
        Game {
            player,
            enemies,
            stage: 0,
        }
    }
    
    pub fn run(&mut self) {
        // A simple narrative for Rusty's adventure.
        let narrative = [
            "Rusty started his adventure, leaving his comfort zone to brave the wild.",
            "After defeating an enemy, Rusty reached a mysterious forest, it seemed oddly quiet...",
            "Upon defeating another enemy, Rusty found himself standing before a gloomy cave entrance..."
        ]; 
    
        let mut action = String::new();
io::stdin().read_line(&mut action).unwrap();
let items = vec![
Item::new(String::from("Healing Potion"), 20, 5),
Item::new(String::from("Super Healing Potion"), 50, 10),
];

match action.trim().parse::<i32>() {
    // Attack or flee cases, etc.
    Ok(3) => {  // Go to shop
        println!("Available items:");
        for (i, item) in items.iter().enumerate() {
            println!("{}. {}: {} gold", i+1, item.name, item.cost);
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse::<usize>() {
            Ok(i) if i > 0 && i <= items.len() => {
                let item = &items[i-1];
                if self.player.gold >= item.cost {
                    self.player.gold -= item.cost;
                    self.player.inventory.push(item.clone());
                    println!("You bought a {}", item.name);
                } else {
                    println!("You don't have enough gold");
                } 
            }
            _ => println!("Invalid choice"), 
        }
    },
    _ => println!("Invalid action"),
}
    }
}