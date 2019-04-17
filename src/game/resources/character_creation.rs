use liblurk::protocol::protocol_message::Character;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default)]
pub struct CharacterCreation(pub HashMap<Uuid, CharacterCreateItem>);

pub struct CharacterCreateItem {
    pub client_id: Uuid,
    pub character_packet: Character,
    pub submitted: bool,
}
