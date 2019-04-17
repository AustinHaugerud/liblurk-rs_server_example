use game::actor::components::combat::Combat;
use game::actor::components::health::Health;
use game::actor::components::kind::Kind;
use game::actor::components::located::Located;
use game::location::components::occupants::Occupants;
use game::resources::fight_events::FightEvents;
use rand::thread_rng;
use rand::Rng;
use specs::prelude::*;

pub struct PlayerFightSystem;

impl<'a> System<'a> for PlayerFightSystem {
    type SystemData = (
        Write<'a, FightEvents>,
        WriteStorage<'a, Health>,
        ReadStorage<'a, Combat>,
        ReadStorage<'a, Located>,
        ReadStorage<'a, Occupants>,
        ReadStorage<'a, Kind>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut fight_events, mut health, combat, located, occupants, kind) = data;

        for event in fight_events.0.drain(..) {
            let location = located
                .get(event.initiator)
                .expect("Bug: Fight initiator not located.");
            let occupants = occupants
                .get(location.room)
                .expect("Bug: Room missing occupancy component.");
            let mut monsters = vec![];
            for (name, entity) in occupants.tenants.iter() {
                let entity_kind = kind.get(*entity).expect("Bug: Entity has no kind.");
                if entity_kind.is_monster {
                    monsters.push(entity);
                }
            }

            if !monsters.is_empty() {
                let mut rng = thread_rng();
                let index = rng.gen_range(0, monsters.len());
                let fight_target = monsters[index];

                let player_combat_stats = combat
                    .get(event.initiator)
                    .expect("Bug: Player has no combat component.");
                let monster_combat_stats = combat
                    .get(*fight_target)
                    .expect("Bug: Monster has no combat component.");

                let damage = 100f32
                    * (player_combat_stats.attack.powi(2) / monster_combat_stats.defense.powi(2));
                let mut monster_health = health
                    .get_mut(*fight_target)
                    .expect("Bug: Monster has no health component.");
                monster_health.reduce(damage);
            }
        }
    }
}
