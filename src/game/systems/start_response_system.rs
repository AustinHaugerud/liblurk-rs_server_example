use game::components::entity::{Attack, Defense, Description, Factions, Health, Location, MaxHealth, Name, PlayerId, Regeneration, Gold};
use game::components::location::ContainedEntities;
use game::resources::character_prep::CharacterPrep;
use game::resources::events::{ChangeRoomEvent, CharacterEvent, StartEvents};
use game::resources::global_name_registry::GlobalNameRegistry;
use game::resources::move_task::{MoveTask, MoveTasks};
use game::resources::start_location::StartLocation;
use game::resources::start_registry::StartRegistry;
use game::types::GameConstants;
use liblurk::protocol::protocol_message::{Character, Error, LurkMessage};
use liblurk::server::server_access::WriteContext;
use liblurk::server::write_queue::enqueue_write;
use specs::prelude::*;
use uuid::Uuid;

pub const SYS_START_RESPONSE: &'static str = "__Start_Response_System__";
pub const SYS_START_RESPONSE_DEPS: &'static [&str] = &[];

pub struct StartResponseSystem;

impl<'a> System<'a> for StartResponseSystem {
    type SystemData = (
        Write<'a, StartEvents>,
        Write<'a, MoveTasks>,
        Write<'a, CharacterPrep>,
        Write<'a, GlobalNameRegistry>,
        Write<'a, StartRegistry>,
        Read<'a, StartLocation>,
        Read<'a, Option<WriteContext>>,
        Read<'a, GameConstants>,
        WriteStorage<'a, ContainedEntities>,
        WriteStorage<'a, Location>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut start_events,
            mut move_tasks,
            mut character_prep,
            mut global_name_registry,
            mut start_registry,
            start_location,
            write_context,
            constants,
            mut contained_entities,
            mut location,
            entities,
            updater,
        ) = data;

        let write = write_context
            .as_ref()
            .expect("Bug: Write context not present.")
            .clone();

        while let Some(event) = start_events.0.pop() {
            let client_id = event.initiator;
            let start_loc = start_location.0.expect("Bug: Start location not present.");

            if start_registry.0.contains(&event.initiator) {
                let error = Error::other(String::from("You have already started."))
                    .expect("Bug: Invalid error packet.");
                enqueue_write(write.clone(), LurkMessage::Error(error), client_id);
            } else {
                if let Some(submission) = character_prep.0.remove(&client_id) {
                    let character_packet =
                        get_character_packet(&submission, &constants, start_loc, client_id);

                    let entity = entities.create();
                    updater.insert(entity, PlayerId(client_id));
                    updater.insert(entity, Name(submission.name));
                    updater.insert(entity, Attack(submission.attack));
                    updater.insert(entity, Defense(submission.defense));
                    updater.insert(entity, Regeneration(submission.regen));
                    updater.insert(entity, MaxHealth(constants.init_health));
                    updater.insert(entity, Health(constants.init_health as f32));
                    updater.insert(entity, Gold(0));
                    updater.insert(entity, Location(start_loc));
                    updater.insert(entity, Description(submission.description));
                    updater.insert(entity, Factions(vec![(String::from("Civil"), 1.0)]));

                    let start_room_container = contained_entities
                        .get_mut(start_loc)
                        .expect("Bug: Locations not built.");
                    start_room_container.0.push(entity);
                } else {
                    let error = Error::not_ready(String::from(
                        "You do not have a ready character profile submitted.",
                    ))
                    .expect("Bug: Invalid error packet.");
                    enqueue_write(write.clone(), LurkMessage::Error(error), client_id);
                }
            }
        }
    }
}

fn get_character_packet(
    submission: &CharacterEvent,
    constants: &GameConstants,
    start_loc: Entity,
    client_id: Uuid,
) -> Character {
    unimplemented!()
}
