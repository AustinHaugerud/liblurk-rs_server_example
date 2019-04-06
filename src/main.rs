extern crate liblurk;
extern crate rand;
extern crate uuid;
#[macro_use]
extern crate nickel;

extern crate specs;

mod game;

use uuid::Uuid;

use liblurk::protocol::protocol_message::*;
use liblurk::server::server::{
    LurkServerError, Server, ServerCallbacks, ServerEventContext, UpdateContext,
};

struct ExampleServer {

}

impl ExampleServer {
    fn new() -> ExampleServer {
        unimplemented!()
    }
}

impl ServerCallbacks for ExampleServer {
    fn on_connect(&mut self, context: &mut ServerEventContext) -> LurkServerError {
        unimplemented!()
    }

    fn on_disconnect(&mut self, client_id: &Uuid) {
        unimplemented!()
    }

    fn on_message(
        &mut self,
        context: &mut ServerEventContext,
        message: &Message,
    ) -> LurkServerError {
        unimplemented!()
    }

    fn on_change_room(
        &mut self,
        context: &mut ServerEventContext,
        change_room: &ChangeRoom,
    ) -> LurkServerError {
        unimplemented!()
    }

    fn on_fight(&mut self, context: &mut ServerEventContext, _: &Fight) -> LurkServerError {
        unimplemented!()
    }

    fn on_pvp_fight(&mut self, context: &mut ServerEventContext, _: &PvpFight) -> LurkServerError {
        unimplemented!()
    }

    fn on_loot(&mut self, context: &mut ServerEventContext, loot: &Loot) -> LurkServerError {
        unimplemented!()
    }

    fn on_start(&mut self, context: &mut ServerEventContext, _: &Start) -> LurkServerError {
        unimplemented!()
    }

    fn on_character(
        &mut self,
        context: &mut ServerEventContext,
        character: &Character,
    ) -> LurkServerError {
        unimplemented!()
    }

    fn on_leave(&mut self, client_id: &Uuid) -> LurkServerError {
        unimplemented!()
    }

    fn update(&mut self, context: &UpdateContext) {
        unimplemented!()
    }
}

fn main() {
    unimplemented!()
}
