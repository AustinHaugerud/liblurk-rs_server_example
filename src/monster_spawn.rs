use entity::Entity;

pub trait MonsterSpawn {
    fn spawn_monsters(&mut self) -> Vec<Entity>;
}

pub mod monster_spawners {
    use super::MonsterSpawn;
    use entity::Entity;
    use rand::{thread_rng, Rng};

    fn spider_spawner() -> Box<MonsterSpawn> {
        Box::new(SpiderSpawner {})
    }

    fn derry_spawner() -> Box<MonsterSpawn> {
        Box::new(DerrySpawner {})
    }

    fn creepy_uncle_spawner() -> Box<MonsterSpawn> {
        Box::new(CreepyUncleSpawner {})
    }

    fn mean_butler_spawner() -> Box<MonsterSpawn> {
        Box::new(MeanButlerSpawner {})
    }

    fn honey_badger_spawner() -> Box<MonsterSpawn> {
        Box::new(HoneyBadgerSpawner {})
    }

    fn mole_people_spawner(level: MolePeopleLevel) -> Box<MonsterSpawn> {
        Box::new(MolePeopleSpawner { level })
    }

    struct SpiderSpawner;

    impl SpiderSpawner {
        fn spawn_small_spider() -> Entity {
            let mut gen = thread_rng();
            Entity {
                name: String::from("Small Spider"),
                attack: 10,
                defense: 75,
                regen: 5,
                health: 50,
                gold: gen.gen_range(5u16, 25u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("A small spider, probably can only you hurt you a little. They're nimble however!"),
            }
        }

        fn spawn_medium_spider() -> Entity {
            let mut gen = thread_rng();
            Entity {
                name: String::from("Spider"),
                attack: 30,
                defense: 60,
                regen: 10,
                health: 125,
                gold: gen.gen_range(20u16, 50u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("A kind of big spider, it'd probably hurt if it bit you."),
            }
        }

        fn spawn_large_spider() -> Entity {
            let mut gen = thread_rng();
            Entity {
                name: String::from("Large Spider"),
                attack: 75,
                defense: 50,
                regen: 25,
                health: 200,
                gold: gen.gen_range(60u16, 150u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("I don't think your shoe is big enough for this."),
            }
        }

        fn spawn_randy_spider() -> Entity {
            let mut gen = thread_rng();
            Entity {
                name: String::from("Big Randy the Smackdown Spider"),
                attack: 200,
                defense: 125,
                regen: 50,
                health: 750,
                gold: gen.gen_range(300u16, 500u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("Big Randy gives fools the smackdown."),
            }
        }

        fn spawn_spider() -> Entity {
            let mut gen = thread_rng();

            let val = gen.gen_range(0u16, 1000u16);

            if val <= 500 {
                return SpiderSpawner::spawn_small_spider();
            } else if val <= 700 {
                return SpiderSpawner::spawn_medium_spider();
            } else if val <= 900 {
                return SpiderSpawner::spawn_large_spider();
            }
            SpiderSpawner::spawn_randy_spider()
        }
    }

    impl MonsterSpawn for SpiderSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            let mut gen = thread_rng();
            let num_spiders = gen.gen_range(3u8, 8u8);

            let mut result: Vec<Entity> = vec![];

            for i in 0..num_spiders {
                result.push(SpiderSpawner::spawn_spider());
            }

            result
        }
    }

    struct DerrySpawner;

    impl MonsterSpawn for DerrySpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            vec![
                Entity {
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
                },
            ]
        }
    }

    struct CreepyUncleSpawner;

    impl MonsterSpawn for CreepyUncleSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            let mut gen = thread_rng();
            vec![
                Entity {
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
                },
            ]
        }
    }

    struct MeanButlerSpawner;

    impl MonsterSpawn for MeanButlerSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            let mut gen = thread_rng();
            vec![
                Entity {
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
                },
            ]
        }
    }

    struct HoneyBadgerSpawner;

    impl MonsterSpawn for HoneyBadgerSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            let mut gen = thread_rng();
            vec![
                Entity {
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
                },
            ]
        }
    }

    struct MolePeopleSpawner {
        level: MolePeopleLevel,
    }

    pub enum MolePeopleLevel {
        Low,
        Mid,
        Hard,
    }

    impl MolePeopleSpawner {
        // 30 percent chance
        fn spawn_mole_grunt() -> Entity {
            let mut gen = thread_rng();
            Entity {
                name: String::from("Mole People Grunt"),
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
            }
        }

        // 25 percent chance
        fn spawn_mole_guard() -> Entity {
            let mut gen = thread_rng();
            Entity {
                name: String::from("Mole People Guard"),
                attack: 75,
                defense: 110,
                regen: 30,
                health: 175,
                gold: gen.gen_range(35u16, 100u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("A guard of the mole people."),
            }
        }

        // 10 percent chance
        fn spawn_mole_priest() -> Entity {
            let mut gen = thread_rng();
            Entity {
                name: String::from("Mole People Priest"),
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
            }
        }

        // 10 percent chance
        fn spawn_fat_mole() -> Entity {
            let mut gen = thread_rng();
            Entity {
                name: String::from("Fat Mole Person"),
                attack: 100,
                defense: 300,
                regen: 100,
                health: 500,
                gold: gen.gen_range(150u16, 250u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("A puss ridden mole person of great girth."),
            }
        }

        // 10 percent chance
        fn spawn_mole_warrior() -> Entity {
            let mut gen = thread_rng();
            Entity {
                name: String::from("Mole People Warrior"),
                attack: 200,
                defense: 200,
                regen: 50,
                health: 325,
                gold: gen.gen_range(125u16, 200u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("A warrior of the mole people."),
            }
        }

        // 5 percent chance
        fn spawn_mole_high_priest() -> Entity {
            let mut gen = thread_rng();
            Entity {
                name: String::from("Mole People High Priest"),
                attack: 300,
                defense: 500,
                regen: 200,
                health: 750,
                gold: gen.gen_range(400u16, 600u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("One of the great high priests of the mole people."),
            }
        }

        // 5 percent chance
        fn spawn_mole_goliath() -> Entity {
            let mut gen = thread_rng();
            Entity {
                name: String::from("Mole Goliath"),
                attack: 500,
                defense: 300,
                regen: 250,
                health: 1250,
                gold: gen.gen_range(500u16, 800u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("A hulking mole goliath."),
            }
        }

        // 5 percent
        fn spawn_mole_queen() -> Entity {
            let mut gen = thread_rng();
            Entity {
                name: String::from("Mole People Queen"),
                attack: 50,
                defense: 1000,
                regen: 400,
                health: 1750,
                gold: gen.gen_range(1000u16, 1200u16),
                location: 0,
                alive: true,
                monster: true,
                desc: String::from("A disgusting mother of the mole people, she'll spawn minions to guard her until she's dead!"),
            }
        }

        fn spawn_mole_person(level: MolePeopleLevel) -> Entity {
            let val = thread_rng().gen_range(0u16, 100u16);

            match level {
                MolePeopleLevel::Low => {
                    if val <= 60 {
                        return MolePeopleSpawner::spawn_mole_grunt();
                    } else {
                        return MolePeopleSpawner::spawn_mole_guard();
                    }
                }
                MolePeopleLevel::Mid => {
                    if val <= 45 {
                        return Entity {
                            name: String::new(),
                            attack: 0,
                            defense: 0,
                            regen: 0,
                            health: 0,
                            gold: 0,
                            location: 0,
                            alive: false,
                            monster: false,
                            desc: String::new(),
                        };
                    } else if val <= 70 {
                        return Entity {
                            name: String::new(),
                            attack: 0,
                            defense: 0,
                            regen: 0,
                            health: 0,
                            gold: 0,
                            location: 0,
                            alive: false,
                            monster: false,
                            desc: String::new(),
                        };
                    } else if val <= 80 {
                        return Entity {
                            name: String::new(),
                            attack: 0,
                            defense: 0,
                            regen: 0,
                            health: 0,
                            gold: 0,
                            location: 0,
                            alive: false,
                            monster: false,
                            desc: String::new(),
                        };
                    } else if val <= 90 {
                        return Entity {
                            name: String::new(),
                            attack: 0,
                            defense: 0,
                            regen: 0,
                            health: 0,
                            gold: 0,
                            location: 0,
                            alive: false,
                            monster: false,
                            desc: String::new(),
                        };
                    } else {
                        return Entity {
                            name: String::new(),
                            attack: 0,
                            defense: 0,
                            regen: 0,
                            health: 0,
                            gold: 0,
                            location: 0,
                            alive: false,
                            monster: false,
                            desc: String::new(),
                        };
                    }
                }
                MolePeopleLevel::Hard => {
                    if val <= 30 {
                        return Entity {
                            name: String::new(),
                            attack: 0,
                            defense: 0,
                            regen: 0,
                            health: 0,
                            gold: 0,
                            location: 0,
                            alive: false,
                            monster: false,
                            desc: String::new(),
                        };
                    } else if val <= 55 {
                        return Entity {
                            name: String::new(),
                            attack: 0,
                            defense: 0,
                            regen: 0,
                            health: 0,
                            gold: 0,
                            location: 0,
                            alive: false,
                            monster: false,
                            desc: String::new(),
                        };
                    } else if val <= 65 {
                        return Entity {
                            name: String::new(),
                            attack: 0,
                            defense: 0,
                            regen: 0,
                            health: 0,
                            gold: 0,
                            location: 0,
                            alive: false,
                            monster: false,
                            desc: String::new(),
                        };
                    } else if val <= 75 {
                        return Entity {
                            name: String::new(),
                            attack: 0,
                            defense: 0,
                            regen: 0,
                            health: 0,
                            gold: 0,
                            location: 0,
                            alive: false,
                            monster: false,
                            desc: String::new(),
                        };
                    } else if val <= 85 {
                        return Entity {
                            name: String::new(),
                            attack: 0,
                            defense: 0,
                            regen: 0,
                            health: 0,
                            gold: 0,
                            location: 0,
                            alive: false,
                            monster: false,
                            desc: String::new(),
                        };
                    } else if val <= 90 {
                        return Entity {
                            name: String::new(),
                            attack: 0,
                            defense: 0,
                            regen: 0,
                            health: 0,
                            gold: 0,
                            location: 0,
                            alive: false,
                            monster: false,
                            desc: String::new(),
                        };
                    } else if val <= 85 {
                        return Entity {
                            name: String::new(),
                            attack: 0,
                            defense: 0,
                            regen: 0,
                            health: 0,
                            gold: 0,
                            location: 0,
                            alive: false,
                            monster: false,
                            desc: String::new(),
                        };
                    } else {
                        return Entity {
                            name: String::new(),
                            attack: 0,
                            defense: 0,
                            regen: 0,
                            health: 0,
                            gold: 0,
                            location: 0,
                            alive: false,
                            monster: false,
                            desc: String::new(),
                        };
                    }
                }
            }
        }
    }

    impl MonsterSpawn for MolePeopleSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            unimplemented!()
        }
    }

    struct DerryHomonculusSpawner;

    impl MonsterSpawn for DerryHomonculusSpawner {
        fn spawn_monsters(&mut self) -> Vec<Entity> {
            let mut gen = thread_rng();
            vec![
                Entity {
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
                },
            ]
        }
    }
}
