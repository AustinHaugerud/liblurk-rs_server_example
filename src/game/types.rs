use ron::de::from_reader;
use serde::Deserialize;

//////////////////////////////////

#[derive(Debug, Deserialize, Clone)]
pub struct GameConstants {
    pub regeneration_factor: f32,
    pub attack_cost: u16,
    pub defense_cost: u16,
    pub regeneration_cost: u16,
    pub health_cost: u16,
    pub init_health: i16,

    pub stat_limit: u16,
    pub init_points: u16,
    pub game_description: String,

    pub starting_location: String,
}

impl Default for GameConstants {
    fn default() -> Self {
        GameConstants {
            regeneration_factor: 0.0001,
            attack_cost: 1,
            defense_cost: 1,
            regeneration_cost: 1,
            health_cost: 1,
            init_health: 10,
            stat_limit: std::u16::MAX,
            init_points: 100,
            game_description: String::from("Default game description"),
            starting_location: String::new(),
        }
    }
}

//////////////////////////////////

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum StoneOffering {
    Attack,
    Defense,
    Regeneration,
    MaxHealth,
    Telepathy,     // Message anyone on the map from anywhere
    Teleportation, // Any location you've ever visited you may go to instantly
    Celerity,      // You have a chance to completely dodge an attack
    Protean,       // You heal
}

#[derive(Debug, Deserialize, Clone)]
pub struct Stone {
    pub name: String,
    pub offering: StoneOffering,
}

//////////////////////////////////

#[derive(Debug, Deserialize)]
pub struct NPCBehaviour {
    pub hostile: bool,
    pub greedy: bool,
    pub roam_activity: f32,
    pub pursue_aggression: f32,
    pub courage: f32,
}

#[derive(Debug, Deserialize)]
pub struct NPCEntity {
    pub name: String,
    pub attack: u16,
    pub defense: u16,
    pub regeneration: u16,
    pub max_health: i16,
    pub description: String,
    pub factions: Vec<(String, f32)>,
    pub behaviour_factors: NPCBehaviour,
}

//////////////////////////////////

#[derive(Debug, Deserialize)]
pub struct Location {
    pub name: String,
    pub description: String,
    pub factions: Vec<String>,
    pub stones: Vec<Stone>,
    pub connections: Vec<String>,
}
