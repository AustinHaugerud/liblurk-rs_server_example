use game::resources::character_prep::CharacterPrep;
use game::resources::events::{CharacterEvent, CharacterEvents};
use game::resources::global_name_registry::GlobalNameRegistry;
use game::resources::start_registry::StartRegistry;
use game::types::GameConstants;
use liblurk::protocol::protocol_message::{Accept, Character, Error, LurkMessage, CHARACTER_TYPE};
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
        Read<'a, StartRegistry>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut character_events,
            mut character_prep,
            mut global_name_registry,
            constants,
            write_context,
            start_registry,
        ) = data;

        let write = write_context
            .as_ref()
            .expect("Bug: Write context not present.")
            .clone();

        while let Some(event) = character_events.0.pop() {
            let client_id = event.initiator;

            let last_submission_name = character_prep.0.get(&client_id).map(|s| &s.name).cloned();

            if start_registry.0.contains(&event.initiator) {
                let error = Error::other(String::from("You have already started the game."))
                    .expect("Bug: Invalid error packet.");
                enqueue_write(write.clone(), LurkMessage::Error(error), client_id);
                continue;
            }

            let fair = is_fair_submission(&event, &constants);
            let name_free = !global_name_registry.0.contains(&event.name);

            if fair && name_free {
                if let Ok(character_packet) = get_character_response(&event) {
                    global_name_registry.0.insert(event.name.clone());
                    character_prep.0.insert(client_id, event);
                    let accept = Accept::new(CHARACTER_TYPE);
                    enqueue_write(write.clone(), LurkMessage::Accept(accept), client_id);
                    enqueue_write(
                        write.clone(),
                        LurkMessage::Character(character_packet),
                        client_id,
                    );
                } else {
                    if let Some(name) = last_submission_name {
                        global_name_registry.0.remove(&name);
                    }
                    character_prep.0.remove(&event.initiator);
                    let error =
                        Error::stat_error(String::from("Your character packet was invalid."))
                            .expect("Bug: Invalid error packet.");
                    enqueue_write(write.clone(), LurkMessage::Error(error), client_id);
                }
            } else {
                if let Some(name) = last_submission_name {
                    global_name_registry.0.remove(&name);
                }
                character_prep.0.remove(&event.initiator);
                let error = get_submission_error(fair, name_free);
                enqueue_write(write.clone(), LurkMessage::Error(error), client_id);
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
        Error::stat_error(String::from(
            "This name is already taken. Also please allocate your points properly..",
        ))
        .expect("Bug: Invalid error packet.")
    } else if !fair && name_free {
        Error::stat_error(String::from("Please allocate your points properly."))
            .expect("Bug: Invalid error packet.")
    } else if fair && !name_free {
        Error::other(String::from("This name is already taken."))
            .expect("Bug: Invalid error packet.")
    } else {
        panic!("Bug: Non error state given.");
    }
}

fn get_character_response(event: &CharacterEvent) -> Result<Character, ()> {
    Character::new(
        event.name.clone(),
        false,
        true,
        false,
        false,
        true,
        event.attack,
        event.defense,
        event.regen,
        0,
        0,
        0,
        event.description.clone(),
    )
}
