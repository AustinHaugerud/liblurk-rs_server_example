use entity::Entity;
use std::sync::Arc;

pub trait MonsterSpawn {
    fn spawn_monsters(&mut self) -> Vec<Entity>;
}

pub type MonsterSpawner = Box<MonsterSpawn + Send>;

pub mod monster_spawners {
    use std::sync::Arc;
    use super::MonsterSpawner;
    use super::MonsterSpawn;
    use entity::Entity;
    use rand::{thread_rng, Rng};

    pub fn spider_spawner() -> MonsterSpawner {
        Box::new(SpiderSpawner { counter: 0 })
    }

    pub fn derry_spawner() -> MonsterSpawner {
        Box::new(DerrySpawner {})
    }

    pub fn creepy_uncle_spawner() -> MonsterSpawner {
        Box::new(CreepyUncleSpawner {})
    }

    pub fn mean_butler_spawner() -> MonsterSpawner {
        Box::new(MeanButlerSpawner {})
    }

    pub fn honey_badger_spawner() -> MonsterSpawner {
        Box::new(HoneyBadgerSpawner {})
    }

    pub fn pit_of_queens_spawner() -> MonsterSpawner {
        Box::new(PitOfQueensSpawner {})
    }

    pub fn mole_people_spawner(
        level: MolePeopleLevel,
        (min_pop, max_pop): (u8, u8),
    ) -> MonsterSpawner {
        Box::new(MolePeopleSpawner {
            level,
            pop_range: (min_pop, max_pop),
            counter: 0,
        })
    }

    pub fn mole_high_priest_spawner() -> MonsterSpawner {
        Box::new(MoleHighPriestSpawner {})
    }

    pub fn great_mole_goliath_spawner() -> MonsterSpawner {
        Box::new(GreatMoleGoliathSpawner {})
    }

    pub fn mole_queen_spawner() -> MonsterSpawner {
        Box::new(MoleQueenSpawner {})
    }

    pub fn homonculus_spawner() -> MonsterSpawner {
        Box::new(DerryHomonculusSpawner {})
    }

    pub fn composite_spawner(spawners: Vec<MonsterSpawner>) -> MonsterSpawner {
        Box::new(CompositeSpawner::new(spawners))
    }

    pub struct SpiderSpawner {
        counter: u32,
    }

    impl SpiderSpawner {
        fn number(&mut self) -> u32 {
            self.counter += 1;
            self.counter
        }

        fn spawn_small_spider(&mut self) -> Entity {
            let mut gen = thread_rng();
            Entity {
                update_dirty: true,
                name: String::from(format!("Small Spider {}", self.number())),
                attack: 10,
                defense: 75,
                regen: 5,
                health: 50,
                gold: gen.gen_range(5u16, 25u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("A small spider, probably can only you hurt you a little. They're nimble however!"),
                base_health : 50,
            }
        }

        fn spawn_medium_spider(&mut self) -> Entity {
            let mut gen = thread_rng();
            Entity {
                update_dirty: true,
                name: String::from(format!("Spider {}", self.number())),
                attack: 30,
                defense: 60,
                regen: 10,
                health: 125,
                gold: gen.gen_range(20u16, 50u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("A kind of big spider, it'd probably hurt if it bit you."),
                base_health: 125,
            }
        }

        fn spawn_large_spider(&mut self) -> Entity {
            let mut gen = thread_rng();
            Entity {
                update_dirty: true,
                name: String::from(format!("Large Spider {}", self.number())),
                attack: 75,
                defense: 50,
                regen: 25,
                health: 200,
                gold: gen.gen_range(60u16, 150u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("I don't think your shoe is big enough for this."),
                base_health: 200,
            }
        }

        fn spawn_randy_spider(&mut self) -> Entity {
            let mut gen = thread_rng();
            Entity {
                update_dirty: true,
                name: String::from(format!("Big Randy the Smackdown Spider {}", self.number())),
                attack: 200,
                defense: 125,
                regen: 50,
                health: 750,
                gold: gen.gen_range(300u16, 500u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("Big Randy gives fools the smackdown."),
                base_health: 750,
            }
        }

        fn spawn_spider(&mut self) -> Entity {
            let mut gen = thread_rng();

            let val = gen.gen_range(0u16, 1000u16);

            if val <= 500 {
                return self.spawn_small_spider();
            } else if val <= 700 {
                return self.spawn_medium_spider();
            } else if val <= 900 {
                return self.spawn_large_spider();
            }
            self.spawn_randy_spider()
        }
    }

    impl MonsterSpawn for SpiderSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            let mut gen = thread_rng();
            let num_spiders = gen.gen_range(3u8, 8u8);

            let mut result: Vec<Entity> = vec![];

            for _ in 0..num_spiders {
                result.push(self.spawn_spider());
            }

            result
        }
    }

    pub struct DerrySpawner;

    impl MonsterSpawn for DerrySpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            vec![
                Entity {
                    update_dirty: true,
                    name: String::from("Derry"),
                    attack: 100,
                    defense: 100,
                    regen: 100,
                    health: 200,
                    gold: 0,
                    location: 0,
                    alive: true,
                    monster: true,
                    desc: String::from("He seems to have lost his mind in a caffeine overdose."),
                    base_health: 200,
                },
            ]
        }
    }

    pub struct CreepyUncleSpawner;

    impl MonsterSpawn for CreepyUncleSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            let mut gen = thread_rng();
            vec![
                Entity {
                    update_dirty: true,
                    name: String::from("Creepy Uncle"),
                    attack: 75,
                    defense: 10,
                    regen: 0,
                    health: 200,
                    gold: gen.gen_range(100u16, 200u16),
                    location: 0,
                    alive: true,
                    monster: true,
                    desc: String::from("\"Come give your uncle a hug buddy\""),
                    base_health: 200,
                },
            ]
        }
    }

    pub struct MeanButlerSpawner;

    impl MonsterSpawn for MeanButlerSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            let mut gen = thread_rng();
            vec![
                Entity {
                    update_dirty: true,
                    name: String::from("Mean Butler"),
                    attack: 50,
                    defense: 10,
                    regen: 5,
                    health: 100,
                    gold: gen.gen_range(5u16, 50u16),
                    location: 0,
                    alive: true,
                    monster: true,
                    desc: String::from(
                        "The butler seems to very strongly believe you should be somewhere else.",
                    ),
                    base_health: 100,
                },
            ]
        }
    }

    pub struct HoneyBadgerSpawner;

    impl MonsterSpawn for HoneyBadgerSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            let mut gen = thread_rng();
            vec![
                Entity {
                    update_dirty: true,
                    name: String::from("Honey Badger"),
                    attack: 350,
                    defense: 250,
                    regen: 300,
                    health: 1000,
                    gold: gen.gen_range(500u16, 1250u16),
                    location: 0,
                    alive: true,
                    monster: true,
                    desc: String::from("This is the honey badger."),
                    base_health: 1000,
                },
            ]
        }
    }

    pub struct MolePeopleSpawner {
        level: MolePeopleLevel,
        pop_range: (u8, u8),
        counter: u32,
    }

    #[derive(Clone)]
    pub enum MolePeopleLevel {
        Low,
        Mid,
        Hard,
    }

    impl MolePeopleSpawner {
        fn number(&mut self) -> u32 {
            self.counter += 1;
            self.counter
        }

        // 30 percent chance
        fn spawn_mole_grunt(&mut self) -> Entity {
            let mut gen = thread_rng();
            Entity {
                update_dirty: true,
                name: String::from(format!("Mole People Grunt {}", self.number())),
                attack: 50,
                defense: 100,
                regen: 20,
                health: 150,
                gold: gen.gen_range(25u16, 75u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from(
                    "A grunt committed to the labor of the mole people civilization.",
                ),
                base_health: 150,
            }
        }

        // 25 percent chance
        fn spawn_mole_guard(&mut self) -> Entity {
            let mut gen = thread_rng();
            Entity {
                update_dirty: true,
                name: String::from(format!("Mole People Guard {}", self.number())),
                attack: 75,
                defense: 110,
                regen: 30,
                health: 175,
                gold: gen.gen_range(35u16, 100u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("A guard of the mole people."),
                base_health: 175,
            }
        }

        // 10 percent chance
        fn spawn_mole_priest(&mut self) -> Entity {
            let mut gen = thread_rng();
            Entity {
                update_dirty: true,
                name: String::from(format!("Mole People Priest {}", self.number())),
                attack: 150,
                defense: 200,
                regen: 100,
                health: 250,
                gold: gen.gen_range(125u16, 200u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from(
                    "A priest of the mole people, spreading the glory of The Great Abomination.",
                ),
                base_health: 250,
            }
        }

        // 10 percent chance
        fn spawn_fat_mole(&mut self) -> Entity {
            let mut gen = thread_rng();
            Entity {
                update_dirty: true,
                name: String::from(format!("Fat Mole Person {}", self.number())),
                attack: 100,
                defense: 300,
                regen: 100,
                health: 500,
                gold: gen.gen_range(150u16, 250u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("A puss ridden mole person of great girth."),
                base_health: 500,
            }
        }

        // 10 percent chance
        fn spawn_mole_warrior(&mut self) -> Entity {
            let mut gen = thread_rng();
            Entity {
                update_dirty: true,
                name: String::from(format!("Mole People Warrior {}", self.number())),
                attack: 200,
                defense: 200,
                regen: 50,
                health: 325,
                gold: gen.gen_range(125u16, 200u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("A warrior of the mole people."),
                base_health: 325,
            }
        }

        // 5 percent chance
        fn spawn_mole_high_priest(&mut self) -> Entity {
            let mut gen = thread_rng();
            Entity {
                update_dirty: true,
                name: String::from(format!("Mole People High Priest {}", self.number())),
                attack: 300,
                defense: 500,
                regen: 200,
                health: 750,
                gold: gen.gen_range(400u16, 600u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("One of the great high priests of the mole people."),
                base_health: 750,
            }
        }

        // 5 percent chance
        fn spawn_mole_goliath(&mut self) -> Entity {
            let mut gen = thread_rng();
            Entity {
                update_dirty: true,
                name: String::from(format!("Mole Goliath {}", self.number())),
                attack: 500,
                defense: 500,
                regen: 250,
                health: 1250,
                gold: gen.gen_range(500u16, 800u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("A hulking mole goliath."),
                base_health: 1250,
            }
        }

        // 5 percent
        fn spawn_mole_queen(&mut self) -> Entity {
            let mut gen = thread_rng();
            Entity {
                update_dirty: true,
                name: String::from(format!("Mole People Queen {}", self.number())),
                attack: 400,
                defense: 1000,
                regen: 400,
                health: 1750,
                gold: gen.gen_range(1000u16, 1200u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("A disgusting mother of the mole people, she'll spawn minions to guard her until she's dead!"),
                base_health: 1750,
            }
        }

        fn spawn_mole_person(&mut self, level: &MolePeopleLevel) -> Entity {
            let val = thread_rng().gen_range(0u16, 100u16);

            match *level {
                MolePeopleLevel::Low => {
                    if val <= 60 {
                        return self.spawn_mole_grunt();
                    } else {
                        return self.spawn_mole_guard();
                    }
                }
                MolePeopleLevel::Mid => {
                    if val <= 45 {
                        return self.spawn_mole_grunt();
                    } else if val <= 70 {
                        return self.spawn_mole_guard();
                    } else if val <= 80 {
                        return self.spawn_mole_priest();
                    } else if val <= 90 {
                        return self.spawn_fat_mole();
                    } else {
                        return self.spawn_mole_warrior();
                    }
                }
                MolePeopleLevel::Hard => {
                    if val <= 30 {
                        return self.spawn_mole_grunt();
                    } else if val <= 55 {
                        return self.spawn_mole_guard();
                    } else if val <= 65 {
                        return self.spawn_mole_priest();
                    } else if val <= 75 {
                        return self.spawn_fat_mole();
                    } else if val <= 85 {
                        return self.spawn_mole_warrior();
                    } else if val <= 90 {
                        return self.spawn_mole_high_priest();
                    } else if val <= 95 {
                        return self.spawn_mole_goliath();
                    } else {
                        return self.spawn_mole_queen();
                    }
                }
            }
        }
    }

    impl MonsterSpawn for MolePeopleSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            let level = self.level.clone();

            let mut result = vec![];

            let (min_moles, max_moles) = self.pop_range;
            let num_moles = thread_rng().gen_range(min_moles, max_moles);

            for _ in 0..num_moles {
                result.push(self.spawn_mole_person(&level));
            }

            result
        }
    }

    pub struct MoleHighPriestSpawner;

    impl MonsterSpawn for MoleHighPriestSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            vec![
                MolePeopleSpawner {
                    level: MolePeopleLevel::Hard,
                    pop_range: (0, 0),
                    counter: 0,
                }.spawn_mole_high_priest(),
            ]
        }
    }

    pub struct GreatMoleGoliathSpawner;

    impl MonsterSpawn for GreatMoleGoliathSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            let mut base = MolePeopleSpawner {
                level: MolePeopleLevel::Hard,
                pop_range: (0, 0),
                counter: 0,
            }.spawn_mole_goliath();
            base.attack = 1000;
            base.defense = 1000;
            base.regen = 300;
            base.name = "Great Mole Goliath".to_string();
            base.desc = "A titanic mole goliath.".to_string();
            vec![base]
        }
    }

    pub struct MoleQueenSpawner;

    impl MonsterSpawn for MoleQueenSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            vec![
                MolePeopleSpawner {
                    level: MolePeopleLevel::Hard,
                    pop_range: (0, 0),
                    counter: 0,
                }.spawn_mole_queen(),
            ]
        }
    }

    pub struct PitOfQueensSpawner;

    impl MonsterSpawn for PitOfQueensSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            let mut result = vec![];
            for i in 0..7 {
                let mut base = MolePeopleSpawner {
                    level: MolePeopleLevel::Hard,
                    pop_range: (0, 0),
                    counter: 0,
                }.spawn_mole_queen();
                base.name = format!("{} {}", base.name, i + 1).to_owned();
                result.push(base);
            }
            result
        }
    }

    pub struct DerryHomonculusSpawner;

    impl MonsterSpawn for DerryHomonculusSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            let mut gen = thread_rng();
            vec![
                Entity {
                    update_dirty: true,
                    name: String::from("Derry's Homonculus"),
                    attack: 1000,
                    defense: 1000,
                    regen: 1000,
                    health: 3000,
                    gold: gen.gen_range(3000u16, 5000u16),
                    location: 0,
                    alive: true,
                    monster: true,
                    desc: String::from(
                        "It's a hideous titanic deformed humanoid, with a resemblance to Derry.",
                    ),
                    base_health: 3000,
                },
            ]
        }
    }

    struct CompositeSpawner {
        impl_spawners: Vec<MonsterSpawner>,
    }

    impl CompositeSpawner {
        pub fn new(spawners: Vec<MonsterSpawner>) -> CompositeSpawner {
            CompositeSpawner {
                impl_spawners: spawners,
            }
        }
    }

    impl MonsterSpawn for CompositeSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            let mut result = vec![];

            for spawner in self.impl_spawners.iter_mut() {
                result.extend(spawner.spawn_monsters());
            }

            result
        }
    }
}
