use specs::{Dispatcher, DispatcherBuilder};
use game::systems::regeneration::{RegenerationSystem, SYS_REGEN, SYS_REGEN_DEPS};
use game::systems::connect_response_system::{ConnectResponseSystem, SYS_CONNECT_RESPONSE, SYS_CONNECT_RESPONSE_DEPS};

pub mod connect_response_system;
pub mod regeneration;

pub fn get_dispatcher<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with(RegenerationSystem, SYS_REGEN, SYS_REGEN_DEPS)
        .with(ConnectResponseSystem, SYS_CONNECT_RESPONSE, SYS_CONNECT_RESPONSE_DEPS)
        .build()
}
