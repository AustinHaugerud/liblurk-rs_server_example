use game::actor::components::gold::Gold;
use game::actor::components::located::Located;
use game::location::components::occupants::Occupants;
use game::resources::loot_events::LootEvents;
use specs::prelude::*;

pub struct LootSystem;

impl<'a> System<'a> for LootSystem {
    type SystemData = (
        Write<'a, LootEvents>,
        WriteStorage<'a, Gold>,
        ReadStorage<'a, Occupants>,
        ReadStorage<'a, Located>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut loot_events, mut gold, occupants, located) = data;

        for loot in loot_events.0.drain(..) {
            let location = located.get(loot.looter).expect("Bug: Looter not located.");
            let occupants = occupants
                .get(location.room)
                .expect("Bug: Room has no occupancy component.");
            if let Some(loot_target) = occupants.tenants.get(&loot.target) {

                let loot_sum =  {
                    let target_gold = gold
                        .get_mut(*loot_target)
                        .expect("Bug: Actor missing gold component");
                    let sum = target_gold.0;
                    target_gold.0 = 0;
                    sum
                };

                let looter_gold = gold
                    .get_mut(loot.looter)
                    .expect("Bug: Looter missing gold component.");
                looter_gold.0 += loot_sum;
            }
        }

        loot_events.0.clear();
    }
}
