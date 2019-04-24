use game::components::entity::{
    Attack, Defense, Description, Dirty, Gold, Health, Location, Name, PlayerId, Regeneration,
};
use game::components::location::{ContainedEntities, Number};
use liblurk::protocol::protocol_message::{Character, LurkMessage};
use liblurk::server::server_access::WriteContext;
use liblurk::server::write_queue::enqueue_write;
use specs::prelude::*;
use std::collections::HashSet;

pub const SYS_RENDER: &'static str = "__Render_System__";
pub const SYS_RENDER_DEPS: &'static [&str] = &[];

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        Read<'a, Option<WriteContext>>,
        ReadStorage<'a, ContainedEntities>,
        ReadStorage<'a, Number>,
        ReadStorage<'a, PlayerId>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Attack>,
        ReadStorage<'a, Defense>,
        ReadStorage<'a, Regeneration>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, Gold>,
        ReadStorage<'a, Location>,
        ReadStorage<'a, Description>,
        WriteStorage<'a, Dirty>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            write_context,
            contained_entities_storage,
            location_number_storage,
            player_id_storage,
            name_storage,
            attack_storage,
            defense_storage,
            regeneration_storage,
            health_storage,
            gold_storage,
            location_storage,
            description_storage,
            mut dirty_storage,
        ) = data;

        let write = write_context
            .as_ref()
            .expect("Bug: Write context not present.")
            .clone();

        for entities_grouping in contained_entities_storage.join() {
            let entities: &HashSet<Entity> = &entities_grouping.0;
            for fixed_entity in entities.iter() {
                // Don't try to write messages to non-player entities.
                if let Some(player_id) = player_id_storage.get(*fixed_entity) {
                    for entity in entities.iter() {
                        if !dirty_storage.get(*entity).unwrap().0 {
                            continue;
                        }

                        let name = name_storage.get(*entity).unwrap();
                        let attack = attack_storage.get(*entity).unwrap();
                        let defense = defense_storage.get(*entity).unwrap();
                        let regen = regeneration_storage.get(*entity).unwrap();
                        let health = health_storage.get(*entity).unwrap();
                        let gold = gold_storage.get(*entity).unwrap();
                        let location = location_storage.get(*entity).unwrap();
                        let description = description_storage.get(*entity).unwrap();
                        let is_monster = player_id_storage.get(*entity).is_none();
                        let is_alive = health.0 > 0f32;
                        let location_num = location_number_storage.get(location.0).unwrap();

                        let character_packet = Character::new(
                            name.0.clone(),
                            is_alive,
                            true,
                            is_monster,
                            true,
                            true,
                            attack.0,
                            defense.0,
                            regen.0,
                            health.0.ceil() as i16,
                            gold.0,
                            location_num.0,
                            description.0.clone(),
                        )
                        .expect("Bug: Invalid character packet in game.");

                        enqueue_write(
                            write.clone(),
                            LurkMessage::Character(character_packet),
                            player_id.0,
                        );

                        dirty_storage.get_mut(*entity).unwrap().0 = false;
                    }
                }
            }
        }
    }
}
