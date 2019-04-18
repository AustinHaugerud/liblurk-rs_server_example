use liblurk::server::callbacks::ServerCallbacks;
use liblurk::server::context::ServerEventContext;
use liblurk::protocol::protocol_message::{Message, ChangeRoom, PvpFight, Loot, Character};
use liblurk::server::server_access::WriteContext;

pub struct Server;

impl ServerCallbacks for Server {
    fn on_connect(&mut self, context: &ServerEventContext) {
        println!("Connection happened.");
    }

    fn on_disconnect(&mut self, context: &ServerEventContext) {
        println!("Disconnect happened.");
    }

    fn on_message(&mut self, context: &ServerEventContext, message: &Message) {
        println!("Got message.");
        println!("Sender: {}", message.sender);
        println!("Receiver: {}", message.receiver);
        println!("Message: {}", message.message);
    }

    fn on_change_room(&mut self, context: &ServerEventContext, change_room: &ChangeRoom) {
        println!("Got change room.");
        println!("Number: {}", change_room.room_number);
    }

    fn on_fight(&mut self, context: &ServerEventContext) {
        println!("Got fight.");
    }

    fn on_pvp_fight(&mut self, context: &ServerEventContext, pvp_fight: &PvpFight) {
        println!("Got pvp fight.");
        println!("Target: {}", pvp_fight.target);
    }

    fn on_loot(&mut self, context: &ServerEventContext, loot: &Loot) {
        println!("Got loot.");
        println!("Target: {}", loot.target);
    }

    fn on_start(&mut self, context: &ServerEventContext) {
        println!("Got start.");
    }

    fn on_character(&mut self, context: &ServerEventContext, character: &Character) {
        println!("On character.");
        println!("Name: {}", character.player_name);
    }

    fn on_leave(&mut self, context: &ServerEventContext) {
        println!("Got leave.");
    }

    fn update(&mut self, context: WriteContext) {
        println!("Update.");
    }
}
