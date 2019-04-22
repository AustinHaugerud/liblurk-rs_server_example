use specs::prelude::*;
use liblurk::server::server_access::WriteContext;
use game::resources::events::ConnectEvents;
use game::types::GameConstants;
use liblurk::protocol::protocol_message::Game;

pub const SYS_CONNECT_RESPONSE: &'static str = "__Connect_Response_System__";
pub const SYS_CONNECT_RESPONSE_DEPS: &'static [&str] = &[];

pub struct ConnectResponseSystem;

impl<'a> System<'a> for ConnectResponseSystem {
    type SystemData = (
        Read<'a, GameConstants>,
        Read<'a, Option<WriteContext>>,
        Write<'a, ConnectEvents>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (constants,
            write_context,
            mut connect_events
        ) = data;

        for event in connect_events.0.drain(..) {
            println!("Processing connect event.");
            let game_packet = Game::new(
                constants.init_points,
                constants.stat_limit,
                constants.game_description.clone()
            ).expect("Bug: Invalid game constants for GAME packet.");

            //write_context.as_ref().unwrap().enqueue_message(game_packet, &event.id);
        }
    }
}
