use game::types::Stone;
use specs::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Number(pub u16);

#[derive(Component)]
#[storage(VecStorage)]
pub struct Name(pub String);

#[derive(Component)]
#[storage(VecStorage)]
pub struct Description(pub String);

#[derive(Component)]
#[storage(VecStorage)]
pub struct ContainedEntities(pub HashSet<Entity>);

#[derive(Component)]
#[storage(VecStorage)]
pub struct Factions(pub Vec<String>);

#[derive(Component)]
#[storage(VecStorage)]
pub struct Stones(pub Vec<Stone>);

#[derive(Component)]
#[storage(VecStorage)]
pub struct ConnectedLocations(pub Vec<Entity>);

pub fn register_components_to_world(world: &mut World) {
    world.register::<Number>();
    world.register::<Name>();
    world.register::<Description>();
    world.register::<ContainedEntities>();
    world.register::<Factions>();
    world.register::<Stones>();
    world.register::<ConnectedLocations>();
}
