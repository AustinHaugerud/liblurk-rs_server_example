use specs::prelude::*;

use game::actor::components::{
    health::Health,
    regeneration::Regeneration,
};

use game::resources::delta_time::DeltaTime;

struct RegenSystem;

impl<'a> System<'a> for RegenSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Health>,
        ReadStorage<'a, Regeneration>
    );

    fn run(&mut self, data : Self::SystemData) {

        let (delta, mut health, regen) = data;

        let delta = delta.0;

        for (health, regen) in (&mut health, &regen).join() {
            let regen_amount = regen.get_regen_amount(&delta) as i16;
            health.add(regen_amount);
        }
    }
}
