pub mod adjacencies;
pub mod description;
pub mod name;
pub mod number;
pub mod occupants;

use specs::World;

pub fn register_location_components(world: &mut World) {
    world.register::<adjacencies::Adjacencies>();
    world.register::<description::Description>();
    world.register::<name::Name>();
    world.register::<number::Number>();
    world.register::<occupants::Occupants>();
}
