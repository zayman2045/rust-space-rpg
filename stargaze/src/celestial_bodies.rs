#[derive(Debug)]
pub struct Planet {
    name: String,
    description: String,
    atmosphere: Atmosphere
}

impl Planet {
    pub fn new(name: String, description: String, atmosphere: Atmosphere) -> Self {
        Self {
            name,
            description,
            atmosphere
        }
    }
}

#[derive(Debug)]
pub enum Atmosphere {
    Pleasant(i32),
    Harsh(i32),
    Unstable(i32)
}

struct Anomaly {
    name: String,
    description: String,
    anomaly_effect: AnomalyEffect
}

enum AnomalyEffect {
    Empowering(i32),
    Oppressing(i32)
}

