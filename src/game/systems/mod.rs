use game::systems::change_room_response_system::{
    ChangeRoomResponseSystem, SYS_CHANGE_ROOM_RESPONSE, SYS_CHANGE_ROOM_RESPONSE_DEPS,
};
use game::systems::character_response_system::{
    CharacterResponseSystem, SYS_CHARACTER_RESPONSE, SYS_CHARACTER_RESPONSE_DEPS,
};
use game::systems::connect_response_system::{
    ConnectResponseSystem, SYS_CONNECT_RESPONSE, SYS_CONNECT_RESPONSE_DEPS,
};
use game::systems::move_system::{MoveSystem, SYS_MOVE, SYS_MOVE_DEPS};
use game::systems::regeneration::{RegenerationSystem, SYS_REGEN, SYS_REGEN_DEPS};
use game::systems::render_system::{RenderSystem, SYS_RENDER, SYS_RENDER_DEPS};
use game::systems::start_response_system::{
    StartResponseSystem, SYS_START_RESPONSE, SYS_START_RESPONSE_DEPS,
};
use specs::{Dispatcher, DispatcherBuilder};
use game::systems::message_response_system::{MessageResponseSystem, SYS_MESSAGE_RESPONSE, SYS_MESSAGE_RESPONSE_DEPS};

pub mod change_room_response_system;
pub mod character_response_system;
pub mod connect_response_system;
pub mod fight_response_system;
pub mod loot_response_system;
pub mod message_response_system;
pub mod move_system;
pub mod pvp_fight_response_system;
pub mod regeneration;
pub mod render_system;
pub mod start_response_system;

pub fn get_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with(RegenerationSystem, SYS_REGEN, SYS_REGEN_DEPS)
        .with(
            ConnectResponseSystem,
            SYS_CONNECT_RESPONSE,
            SYS_CONNECT_RESPONSE_DEPS,
        )
        .with(
            CharacterResponseSystem,
            SYS_CHARACTER_RESPONSE,
            SYS_CHARACTER_RESPONSE_DEPS,
        )
        .with(RenderSystem, SYS_RENDER, SYS_RENDER_DEPS)
        .with(
            StartResponseSystem,
            SYS_START_RESPONSE,
            SYS_START_RESPONSE_DEPS,
        )
        .with(
            ChangeRoomResponseSystem,
            SYS_CHANGE_ROOM_RESPONSE,
            SYS_CHANGE_ROOM_RESPONSE_DEPS,
        )
        .with(MoveSystem, SYS_MOVE, SYS_MOVE_DEPS)
        .with(MessageResponseSystem, SYS_MESSAGE_RESPONSE, SYS_MESSAGE_RESPONSE_DEPS)
        .build()
}
