use game::systems::character_response_system::{
    CharacterResponseSystem, SYS_CHARACTER_RESPONSE, SYS_CHARACTER_RESPONSE_DEPS,
};
use game::systems::connect_response_system::{
    ConnectResponseSystem, SYS_CONNECT_RESPONSE, SYS_CONNECT_RESPONSE_DEPS,
};
use game::systems::regeneration::{RegenerationSystem, SYS_REGEN, SYS_REGEN_DEPS};
use specs::{Dispatcher, DispatcherBuilder};

pub mod character_response_system;
pub mod connect_response_system;
pub mod regeneration;
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
        .build()
}
