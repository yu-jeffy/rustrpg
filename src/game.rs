use std::io;
use std::io::prelude::*;
use crate::character::Character;
use crate::enemy::Enemy;

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
    
        let mut i = 0;
        
        while self.player.hp > 0 && self.stage < self.enemies.len() {
            let enemy = &mut self.enemies[self.stage];
            // Narrate the game story.
            if i < narrative.len() {
                println!("{}", narrative[i]);
            }
            println!(
                "{} encountered {}. What do you want to do, Rusty? 1: Attack, 2: Flee",
                self.player.name, enemy.name
            );
            let mut action = String::new();
            io::stdin().read_line(&mut action).unwrap();
            match action.trim().parse::<i32>() {
                Ok(1) => {  // Attack
                    enemy.hp -= self.player.damage;
                    if enemy.hp > 0 {
                        self.player.hp -= enemy.damage;
                        println!("You hit the enemy for {} damage!", self.player.damage);
                        println!("Enemy hits you for {} damage!", enemy.damage);
                    } else {
                        println!("You hit the enemy for {} damage. The enemy is defeated!", self.player.damage);
                        self.stage += 1;  // Proceed to the next stage
                        i += 1; // Move narrative along with stage
                    }
                    println!("Your current HP: {}. Enemy's current HP: {}", self.player.hp, enemy.hp);
                },
                Ok(2) => {  // Flee
                    println!("You run away safely. But the invaders are still there!");
                    return;
                },
                _ => {
                    println!("Invalid action!");
                },
            }
        }
    
        if self.player.hp > 0 {
            println!("Victory! Our hero Rusty stands victorious over the invaders!");
        } else {
            println!("Defeat... The invaders defeated our brave Rusty");
        }
    }
}