

mod celestial_bodies;
mod characters;

fn main() {
    println!("\nWelcome to Stargaze! Prepare for battle!\n");
    println!("Choose one of the following heroes:\n");
    println!("- Star Pilot\n- Battle Engineer\n- Space Mage\n");

    let input = "Space Mage";


    let player_hero_type = match input {
        "Star Pilot" => {println!("You are now a Star Pilot!");
            characters::heroes::HeroType::StarPilot},
        "Battle Engineer" => {println!("You are now a Battle Engineer!");
        characters::heroes::HeroType::BattleEngineer},
        "Space Mage" => {println!("You are now a Space Mage!");
        characters::heroes::HeroType::SpaceMage},
        _ => panic!("Invalid Input")
    };

    // create instance of home planet
    let player_home_planet = celestial_bodies::Planet::new("Earth".to_string(), "A basic, ordinary planet".to_string(), celestial_bodies::Atmosphere::Pleasant(0));

    // create instance of player card
    let player_card = characters::Card::create_player(player_hero_type, player_home_planet);


}