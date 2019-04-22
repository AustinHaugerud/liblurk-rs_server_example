use game::components::entity::Location;
use game::components::location::ContainedEntities;
use game::resources::character_prep::CharacterPrep;
use game::resources::events::StartEvents;
use game::resources::global_name_registry::GlobalNameRegistry;
use game::resources::start_location::StartLocation;
use specs::prelude::*;

pub struct StartResponseSystem;

impl<'a> System<'a> for StartResponseSystem {
    type SystemData = (
        Write<'a, StartEvents>,
        Write<'a, CharacterPrep>,
        Write<'a, GlobalNameRegistry>,
        Read<'a, StartLocation>,
        WriteStorage<'a, ContainedEntities>,
        WriteStorage<'a, Location>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut start_events,
            mut character_prep,
            mut global_name_registry,
            start_location,
            mut contained_entities,
            mut location,
        ) = data;
    }
}
