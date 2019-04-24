use specs::prelude::*;
use game::resources::move_task::MoveTasks;
use game::components::entity::{Location, PlayerId, Attack, Regeneration, Defense, Health, Gold};
use liblurk::server::server_access::WriteContext;
use game::components::location::{ContainedEntities, Number, ConnectedLocations};
use game::components::entity::Name as EntityName;
use game::components::location::Name as LocationName;
use game::components::entity::Description as EntityDescription;
use game::components::location::Description as LocationDescription;
use liblurk::protocol::protocol_message::{Character, LurkMessage, Room, Connection};
use liblurk::server::write_queue::enqueue_write;

pub const SYS_MOVE: &'static str = "__Move_System__";
pub const SYS_MOVE_DEPS: &'static [&str] = &[];

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        Read<'a, Option<WriteContext>>,
        Write<'a, MoveTasks>,
        WriteStorage<'a, Location>,
        WriteStorage<'a, ContainedEntities>,
        ReadStorage<'a, PlayerId>,
        ReadStorage<'a, EntityName>,
        ReadStorage<'a, Attack>,
        ReadStorage<'a, Defense>,
        ReadStorage<'a, Regeneration>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Gold>,
        ReadStorage<'a, EntityDescription>,
        ReadStorage<'a, Number>,
        ReadStorage<'a, ConnectedLocations>,
        ReadStorage<'a, LocationDescription>,
        ReadStorage<'a, LocationName>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            write_context,
            mut move_tasks,
            mut location_storage,
            mut contained_entities_storage,
            player_id_storage,
            entity_name_storage,
            attack_storage,
            defense_storage,
            regeneration_storage,
            health_storage,
            gold_storage,
            entity_description_storage,
            number_storage,
            connected_locations_storage,
            location_description_storage,
            location_name_storage,
        ) = data;

        let write = write_context.as_ref().expect("Bug: Write context not present.").clone();

        while let Some(move_task) = move_tasks.0.pop() {
            let mover = move_task.mover;
            let target_location = move_task.target;
            let old_location = location_storage.get(mover).expect("Bug: Entity lacks location component.").0;

            location_storage.get_mut(mover).expect("Bug: Entity missing location component.").0 = target_location;
            contained_entities_storage.get_mut(old_location).expect("Bug: Location missing contained entities component.").0.remove(&mover);
            ;
            contained_entities_storage.get_mut(target_location).expect("").0.insert(mover);

            let name = entity_name_storage.get(mover).expect("Bug: Entity missing name component").0.clone();
            let attack = attack_storage.get(mover).expect("Bug: Entity missing attack component").0;
            let defense = defense_storage.get(mover).expect("").0;
            let regen = regeneration_storage.get(mover).expect("").0;
            let health = health_storage.get(mover).expect("").0;
            let gold = gold_storage.get(mover).expect("").0;
            let desc = entity_description_storage.get(mover).expect("").0.clone();
            let room_num = number_storage.get(target_location).expect("").0;
            let mover_character_packet = Character::new(name.clone(), true, true, false, true, true, attack, defense, regen, health.ceil() as i16, gold, room_num, desc).unwrap();
            let packet = LurkMessage::Character(mover_character_packet);

            for entity in contained_entities_storage.get(old_location).expect("Bug: Location lacks contained entities component.").0.iter() {
                if let Some(player_id) = player_id_storage.get(*entity) {
                    let id = player_id.0;
                    enqueue_write(write.clone(), packet.clone(), id);
                }
            }

            if let Some(mover_id) = player_id_storage.get(mover).map(|i| i.0) {
                let curr_location_name = location_name_storage.get(target_location).unwrap().0.clone();
                let curr_location_description = location_description_storage.get(target_location).unwrap().0.clone();

                let curr_location_packet = Room::new(room_num, curr_location_name, curr_location_description).unwrap();
                enqueue_write(write.clone(), LurkMessage::Room(curr_location_packet), mover_id);

                for location in connected_locations_storage.get(target_location).unwrap().0.iter() {
                    let con_loc_num = number_storage.get(*location).unwrap().0;
                    let con_loc_name = location_name_storage.get(*location).unwrap().0.clone();
                    let con_loc_desc = location_description_storage.get(*location).unwrap().0.clone();

                    let connection_packet = Connection::new(con_loc_num, con_loc_name, con_loc_desc).unwrap();
                    enqueue_write(write.clone(), LurkMessage::Connection(connection_packet), mover_id);
                }
            }
        }
    }
}
