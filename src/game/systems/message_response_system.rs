use game::components::entity::{Abilities, Location, Name};
use game::resources::events::MessageEvents;
use game::resources::id_entity_mapping::IdEntityMapping;
use game::resources::id_name_mapping::IdNameMapping;
use game::resources::start_registry::StartRegistry;
use liblurk::protocol::protocol_message::{Error, LurkMessage, Message};
use liblurk::server::server_access::WriteContext;
use liblurk::server::write_queue::enqueue_write;
use specs::prelude::*;

pub const SYS_MESSAGE_RESPONSE: &'static str = "__Message_Response_System__";
pub const SYS_MESSAGE_RESPONSE_DEPS: &'static [&str] = &[];

pub struct MessageResponseSystem;

impl<'a> System<'a> for MessageResponseSystem {
    type SystemData = (
        Read<'a, StartRegistry>,
        Read<'a, Option<WriteContext>>,
        Read<'a, IdNameMapping>,
        Read<'a, IdEntityMapping>,
        Write<'a, MessageEvents>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Location>,
        ReadStorage<'a, Abilities>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            start_registry,
            write_context,
            id_name_mapping,
            id_entity_mapping,
            mut message_events,
            name_storage,
            location_storage,
            abilities_storage,
        ) = data;

        let write = write_context.as_ref().unwrap().clone();

        while let Some(event) = message_events.0.pop() {
            let sender_id = event.initiator;
            if let Some(target_entity_id) = id_name_mapping.get_id(&event.target) {
                if !start_registry.0.contains(&target_entity_id) {
                    let error =
                        Error::not_ready(String::from("You have not started yet.")).unwrap();
                    let packet = LurkMessage::Error(error);
                    enqueue_write(write.clone(), packet, sender_id);
                    continue;
                }

                let sender_entity = *id_entity_mapping.0.get(&sender_id).unwrap();
                let target_entity = *id_entity_mapping.0.get(&target_entity_id).unwrap();

                let sender_loc = location_storage.get(sender_entity).unwrap();
                let target_loc = location_storage.get(target_entity).unwrap();

                let sender_abilities = abilities_storage.get(sender_entity).unwrap();

                // Only telepaths can message people at different locations.
                if !sender_abilities.telepathy && sender_loc.0 != target_loc.0 {
                    let error = Error::other(String::from(
                        "Only telepaths can message people not close by.",
                    ))
                    .unwrap();
                    let packet = LurkMessage::Error(error);
                    enqueue_write(write.clone(), packet, sender_id);
                    continue;
                }

                let sender_name = name_storage.get(sender_entity).unwrap().0.clone();
                let recip_name = event.target;
                let content = event.content;

                let message = Message::new(content, sender_name, recip_name).unwrap();
                let packet = LurkMessage::Message(message);
                enqueue_write(write.clone(), packet, *target_entity_id);
            } else {
                let error = Error::no_target(format!(
                    "'{}' either does not exist or cannot/won't converse with you.",
                    &event.target
                ))
                .unwrap();
                let packet = LurkMessage::Error(error);
                enqueue_write(write.clone(), packet, sender_id);
            }
        }
    }
}
