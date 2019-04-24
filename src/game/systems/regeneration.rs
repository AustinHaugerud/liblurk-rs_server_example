use game::components::entity::{Dirty, Health, MaxHealth, Regeneration};
use game::types::GameConstants;
use specs::prelude::*;

pub const SYS_REGEN: &'static str = "__Regen_System__";
pub const SYS_REGEN_DEPS: &'static [&str] = &[];

pub struct RegenerationSystem;

impl<'a> System<'a> for RegenerationSystem {
    type SystemData = (
        Read<'a, GameConstants>,
        ReadStorage<'a, MaxHealth>,
        ReadStorage<'a, Regeneration>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Dirty>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            constants,
            max_health_storage,
            regeneration_storage,
            mut health_storage,
            mut dirty_storage,
        ) = data;

        for (max_health, regeneration, health, dirty) in (
            &max_health_storage,
            &regeneration_storage,
            &mut health_storage,
            &mut dirty_storage,
        )
            .join()
        {
            if health.0 < max_health.0 as f32 {
                health.0 +=
                    constants.regeneration_factor * regeneration.0 as f32 * max_health.0 as f32;

                if health.0 > max_health.0 as f32 {
                    health.0 = max_health.0 as f32;
                }

                dirty.0 = true;
            }
        }
    }
}
