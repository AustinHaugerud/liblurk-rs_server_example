use game::resources::character_prep::CharacterPrep;
use game::resources::events::{CharacterEvent, CharacterEvents};
use game::resources::global_name_registry::GlobalNameRegistry;
use game::types::GameConstants;
use liblurk::protocol::protocol_message::{Accept, Error, LurkMessage, CHARACTER_TYPE};
use liblurk::server::server_access::WriteContext;
use liblurk::server::write_queue::enqueue_write;
use specs::prelude::*;

pub const SYS_CHARACTER_RESPONSE: &'static str = "__Character_Response_System__";
pub const SYS_CHARACTER_RESPONSE_DEPS: &'static [&str] = &[];

pub struct CharacterResponseSystem;

impl<'a> System<'a> for CharacterResponseSystem {
    type SystemData = (
        Write<'a, CharacterEvents>,
        Write<'a, CharacterPrep>,
        Write<'a, GlobalNameRegistry>,
        Read<'a, GameConstants>,
        Read<'a, Option<WriteContext>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut character_events,
            mut character_prep,
            mut global_name_registry,
            constants,
            write_context,
        ) = data;

        let write = write_context
            .as_ref()
            .expect("Bug: Write context not present.")
            .clone();

        while let Some(event) = character_events.0.pop() {
            let fair = is_fair_submission(&event, &constants);
            let name_free = !global_name_registry.0.contains(&event.name);
            let write_target = event.initiator;

            if fair && name_free {
                character_prep.0.insert(event.initiator, event);
                let accept = Accept::new(CHARACTER_TYPE);
                enqueue_write(write.clone(), LurkMessage::Accept(accept), write_target);
            } else {
                character_prep.0.remove(&event.initiator);
                let error = get_submission_error(fair, name_free);
                enqueue_write(write.clone(), LurkMessage::Error(error), write_target);
            }
        }
    }
}

fn is_fair_submission(submission: &CharacterEvent, constants: &GameConstants) -> bool {
    let cost = submission.attack + submission.defense + submission.regen;

    cost == constants.init_points
}

fn get_submission_error(fair: bool, name_free: bool) -> Error {
    if !fair && !name_free {
        Error::stat_error(String::from("")).expect("Bug: Invalid error packet.")
    } else if !fair && name_free {
        Error::stat_error(String::from("")).expect("Bug: Invalid error packet.")
    } else if fair && !name_free {
        Error::other(String::from("")).expect("Bug: Invalid error packet.")
    } else {
        panic!("Bug: Non error state given.");
    }
}
