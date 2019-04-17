use game::actor::components::combat::Combat;
use game::actor::components::gold::Gold;
use game::actor::components::health::Health;
use game::actor::components::kind::Kind;
use game::actor::components::located::Located;
use game::actor::components::name::Name;
use game::actor::components::regeneration::Regeneration;
use game::location::components::number::Number;
use game::location::components::occupants::Occupants;
use game::resources::feedback::FeedbackItems;
use specs::prelude::*;

pub struct CharacterCrierSystem;

impl<'a> System<'a> for CharacterCrierSystem {
    type SystemData = (
        Write<'a, FeedbackItems>,
        ReadStorage<'a, Combat>,
        ReadStorage<'a, Gold>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Kind>,
        ReadStorage<'a, Located>,
        ReadStorage<'a, Number>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Regeneration>,
        ReadStorage<'a, Occupants>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut feedback,
            combat,
            gold,
            health,
            kind,
            located,
            number,
            name,
            regeneration,
            occupants,
        ) = data;

        for occupant_collection in occupants.join() {}
    }
}
