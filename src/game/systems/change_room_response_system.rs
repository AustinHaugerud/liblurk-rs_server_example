use specs::prelude::*;
use game::resources::events::ChangeRoomEvents;
use game::resources::move_task::{MoveTasks, MoveTask};
use game::components::entity::{Location, PlayerId};
use game::components::location::{ConnectedLocations, Number};
use game::resources::id_name_mapping::IdNameMapping;
use game::resources::id_entity_mapping::IdEntityMapping;
use game::resources::start_registry::StartRegistry;
use liblurk::server::server_access::WriteContext;
use liblurk::protocol::protocol_message::{Error, LurkMessage};
use liblurk::server::write_queue::enqueue_write;

pub struct ChangeRoomResponseSystem;

impl<'a> System<'a> for ChangeRoomResponseSystem {
    type SystemData = (
        Read<'a, Option<WriteContext>>,
        Read<'a, StartRegistry>,
        Read<'a, IdEntityMapping>,
        Write<'a, ChangeRoomEvents>,
        Write<'a, MoveTasks>,
        ReadStorage<'a, PlayerId>,
        ReadStorage<'a, Location>,
        ReadStorage<'a, ConnectedLocations>,
        ReadStorage<'a, Number>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            write_context,
            start_registry,
            id_entity_mapping,
            mut change_room_events,
            mut move_tasks,
            player_id_storage,
            location_storage,
            connected_locations_storage,
            number_storage,
        ) = data;

        let write = write_context.as_ref().expect("Bug: Write context not present.").clone();

        while let Some(event) = change_room_events.0.pop() {
            let client_id = event.initiator;
            if start_registry.0.contains(&client_id) {
                let mover = *id_entity_mapping.0.get(&client_id).expect("Bug: Player id not associated with an entity.");
                let current_room = location_storage.get(mover).expect("Bug: Player is not in placement.").0;

                let connected_rooms = connected_locations_storage.get(current_room).expect("Bug: Room has no connections component.");

                let target_location = {
                    let mut result = None;
                    for location in connected_rooms.0.iter() {
                        let num = number_storage.get(*location).expect("Bug: Location missing number component.");
                        if num.0 == event.room_number {
                            result = Some(*location);
                            break;
                        }
                    }
                    result
                };

                if let Some(target) = target_location {
                    let move_task = MoveTask {
                        mover,
                        target
                    };
                    move_tasks.0.push(move_task);
                }
                else {
                    let error = Error::no_target(String::from("You cannot go there..")).expect("Bug: Invalid error packet.");
                    enqueue_write(write.clone(), LurkMessage::Error(error), client_id);
                }
            }
            else {
                let error = Error::not_ready(String::from("You have not started.")).expect("Bug: Invalid error packet.");
                enqueue_write(write.clone(), LurkMessage::Error(error), client_id);
            }
        }
    }
}
