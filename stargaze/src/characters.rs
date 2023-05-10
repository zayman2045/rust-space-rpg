pub mod enemies;
pub mod heroes;
use crate::celestial_bodies;

#[derive(Debug)]
pub enum Role {
    Hero(heroes::HeroType),
    Enemy(enemies::EnemyType),
    Neutral
}

#[derive(Debug)]
pub struct Card {
    role: Role,
    health: i32,
    attack_power: i32,
    home_planet: celestial_bodies::Planet
}

impl Card {
    pub fn new(role: Role, health: i32, attack_power: i32, home_planet :celestial_bodies::Planet) -> Self {
        Self {
            role,
            health,
            attack_power,
            home_planet
        }
    }

    pub fn create_player(hero_type: heroes::HeroType, home_planet :celestial_bodies::Planet) -> Self {
        Self {
            role: Role::Hero(hero_type),
            health: 100,
            attack_power:100,
            home_planet
        }
    }
}


