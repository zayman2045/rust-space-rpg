mod celestial_bodies;
use celestial_bodies::Planet;


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

// spawn an enemy

// battle the spawned enemy