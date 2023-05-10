use std::io;

mod celestial_bodies;
use celestial_bodies::Planet;

mod characters;
use characters::Card;

// user selects hero type to begin the game
pub fn start_game() -> Card {
    println!("\nWelcome to Stargaze! Prepare for battle!\n");
    println!("Choose one of the following heroes:\n");
    println!("- Star Pilot\n- Battle Engineer\n- Space Mage\n");

    let mut hero_selection = String::new();
    match io::stdin().read_line(&mut hero_selection){
        Ok(input) => input,
        Err(_) => panic!("Failed to select hero")
    };

    let player_hero_type = match hero_selection.trim() {
        "Star Pilot" => {println!("\nYou are now a Star Pilot. Welcome to the fight!");
            characters::heroes::HeroType::StarPilot},
        "Battle Engineer" => {println!("\nYou are now a Battle Engineer. Welcome to the fight!");
        characters::heroes::HeroType::BattleEngineer},
        "Space Mage" => {println!("\nYou are now a Space Mage. Welcome to the fight!");
        characters::heroes::HeroType::SpaceMage},
        _ => panic!("Invalid Input")
    };

    // create instance of home planet
    let player_home_planet = celestial_bodies::Planet::new("Earth".to_string(), "A basic, ordinary planet".to_string(), celestial_bodies::Atmosphere::Pleasant(0));

    // create instance of player card
    characters::Card::create_player(player_hero_type, player_home_planet)

    // TODO: create_planets and pass one as an argument to Planet::new() instead of explicitly defining player_home_planet
}

// create all planets
pub fn create_planets() -> Vec<Planet> {
    let mut planets = Vec::new();

    let earth = Planet::new(
        "Earth".to_string(),
        "A basic, ordinary planet".to_string(),
        celestial_bodies::Atmosphere::Pleasant(0)
    );
        planets.push(earth);

    let cryola = Planet::new(
        "Cryola: The Singing Planet".to_string(),
        "This is a planet that emits a constant and eerie hum that can be heard across the galaxy.".to_string(),
        celestial_bodies::Atmosphere::Unstable(10)
    );
    planets.push(cryola);
    
    let yandar = Planet::new(
        "Yandar: The Glass Planet".to_string(),
        "This is a planet that is entirely covered in a massive sheet of crystal-clear glass, making it a glittering and treacherous landscape to navigate.".to_string(),
        celestial_bodies::Atmosphere::Harsh(-30)
    );
    planets.push(yandar);

    planets
}

// spawn an enemy card
fn spawn_enemy() -> Card {
    todo!()
}

// battle the spawned enemy using rng (card vs card)
// use generics and create HeroCard/EnemyCard types
fn battle(player_card: Card, enemy_card: Card) {
    todo!()
}
