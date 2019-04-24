use specs::prelude::*;
use uuid::Uuid;

/// PLAYER ID
#[derive(Component)]
#[storage(VecStorage)]
pub struct PlayerId(pub Uuid);

/// NAME
#[derive(Component)]
#[storage(VecStorage)]
pub struct Name(pub String);

/// ATTACK
#[derive(Component)]
#[storage(VecStorage)]
pub struct Attack(pub u16);

/// DEFENSE
#[derive(Component)]
#[storage(VecStorage)]
pub struct Defense(pub u16);

/// REGENERATION
#[derive(Component)]
#[storage(VecStorage)]
pub struct Regeneration(pub u16);

/// MAX HEALTH
#[derive(Component)]
#[storage(VecStorage)]
pub struct MaxHealth(pub i16);

/// HEALTH
/// Note, health is stored as f32 to play more friendly
/// with the regeneration system for slow regeneration.
/// When character packets are sent, the health is
/// rounded up.
#[derive(Component)]
#[storage(VecStorage)]
pub struct Health(pub f32);

/// GOLD
#[derive(Component)]
#[storage(VecStorage)]
pub struct Gold(pub u16);

/// LOCATION
#[derive(Component)]
#[storage(VecStorage)]
pub struct Location(pub Entity);

/// DESCRIPTION
#[derive(Component)]
#[storage(VecStorage)]
pub struct Description(pub String);

/// FACTIONS
#[derive(Component)]
#[storage(VecStorage)]
/**
 * Faction Id
 * Loyalty weight
 */
pub struct Factions(pub Vec<(String, f32)>);

/// BEHAVIOUR
#[derive(Component)]
#[storage(VecStorage)]
pub struct BehaviourFactors {
    pub hostile: bool,
    pub greedy: bool,
    pub roam_activity: f32,     // Tendency to move out of combat
    pub pursue_aggression: f32, // Aggression factor when pursuing player
    pub courage: f32,           // Courage factors, also affects pursuing and also fleeing
}

/// IN COMBAT
#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct InCombat;

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Dirty(pub bool);

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Abilities {
    pub telepathy: bool,
    pub teleportation: bool,
}

pub fn register_components_to_world(world: &mut World) {
    world.register::<PlayerId>();
    world.register::<Name>();
    world.register::<Attack>();
    world.register::<Defense>();
    world.register::<Regeneration>();
    world.register::<MaxHealth>();
    world.register::<Health>();
    world.register::<Gold>();
    world.register::<Location>();
    world.register::<Description>();
    world.register::<Factions>();
    world.register::<BehaviourFactors>();
    world.register::<InCombat>();
    world.register::<Dirty>();
    world.register::<Abilities>();
}
