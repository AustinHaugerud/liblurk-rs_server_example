pub mod behaviour;
pub mod client;
pub mod combat;
pub mod gold;
pub mod health;
pub mod kind;
pub mod located;
pub mod name;
pub mod regeneration;
pub mod stat_shop;

use specs::World;

pub fn register_actor_components(world: &mut World) {
    world.register::<behaviour::Behaviour>();
    world.register::<combat::Combat>();
    world.register::<health::Health>();
    world.register::<regeneration::Regeneration>();
    world.register::<gold::Gold>();
    world.register::<located::Located>();
    world.register::<name::Name>();
    world.register::<stat_shop::StatShop>();
    world.register::<kind::Kind>();
    world.register::<client::Client>();
}
