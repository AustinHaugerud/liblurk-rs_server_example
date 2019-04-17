use game::actor::components::client::Client;
use game::actor::components::located::Located;
use game::actor::components::name::Name;
use game::location::components::adjacencies::Adjacencies;
use game::location::components::number::Number;
use game::location::components::occupants::Occupants;
use game::resources::change_room_events::ChangeRoomEvents;
use game::resources::feedback::{FeedbackItem, FeedbackItems};
use liblurk::protocol::protocol_message::Error;
use specs::prelude::*;

pub struct ChangeRoomSystem;

impl<'a> System<'a> for ChangeRoomSystem {
    type SystemData = (
        Write<'a, ChangeRoomEvents>,
        Write<'a, FeedbackItems>,
        WriteStorage<'a, Located>,
        ReadStorage<'a, Number>,
        ReadStorage<'a, Adjacencies>,
        WriteStorage<'a, Occupants>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Client>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut events,
            mut feedback,
            mut located,
            number,
            adjacencies,
            mut occupants,
            name,
            client,
        ) = data;

        for event in events.0.drain(..) {
            let location = located
                .get_mut(event.mover)
                .expect("Bug: Player lacks located component.");
            let adjacent_rooms = adjacencies
                .get(location.room)
                .expect("Bug: Room lacks adjacencies component.");
            if let Some(new_room) = adjacent_rooms.0.iter().find(|val| {
                let num = number
                    .get(**val)
                    .expect("Bug: Room lacks number component.");
                num.0 == event.target_room
            }) {

                let mover_name = name
                    .get(event.mover)
                    .expect("Bug: Player missing name component.");

                {
                    let mut old_room_occupants = occupants
                        .get_mut(event.mover)
                        .expect("Bug: Room missing occupants component.");
                    old_room_occupants.tenants.remove(&mover_name.0);
                }

                {
                    let mut new_room_occupants = occupants
                        .get_mut(*new_room)
                        .expect("Bug: Room missing occupants component.");
                    new_room_occupants
                        .tenants
                        .insert(mover_name.0.clone(), event.mover);
                }

                location.room = *new_room;
            } else {
                let client_id = client
                    .get(event.mover)
                    .expect("Bug: Player entity lacks client component.")
                    .id
                    .clone();
                let err = Error::bad_room("Not an adjacent room.".to_string())
                    .expect("Bug:Invalid bad room error.");
                feedback.0.push(FeedbackItem::new(err, &client_id));
            }
        }
    }
}
