use specs::prelude::*;

use game::actor::components::{
    health::Health,
    combat::Combat,
    regeneration::Regeneration,
};

use game::components::purchase::Purchase;
use game::actor::stat::Stat;

struct PurchaseSystem;

impl<'a> System<'a> for PurchaseSystem {
    type SystemData = (
        WriteStorage<'a, Health>,
        WriteStorage<'a, Combat>,
        WriteStorage<'a, Regeneration>,
        WriteStorage<'a, Purchase>,
    );

    fn run(&mut self, data: Self::SystemData) {
        unimplemented!()
    }
}
