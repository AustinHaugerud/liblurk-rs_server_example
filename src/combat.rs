use entity::Entity;
use std::num;
use rand::{thread_rng, Rand, Rng};

fn get_initiative_values(lentity: &Entity, rentity: &Entity) -> (f32, f32) {
    let lattack = lentity.get_effective_attack() as f32;
    let rattack = rentity.get_effective_attack() as f32;
    let scalar = (lattack.powi(2) + rattack.powi(2)).sqrt();
    (lattack / scalar, rattack / scalar)
}

fn get_hit_chance(attack: u16, defense: u16) -> f32 {
    ((attack as f32 * 0.75f32) / defense as f32).max(0.9f32)
}

fn get_damage(attack: u16, defense: u16) -> f32 {
    let min_damage = (attack as f32 - (defense as f32 * 1.5)).min(attack as f32 * 0.1);
    let max_damage = attack as f32;
    println!("Min dmg: {}", min_damage);
    println!("Max dmg: {}", max_damage);
    thread_rng().gen_range(min_damage, max_damage)
}

pub fn handle_fight(lentity: &mut Entity, rentity: &mut Entity) -> String {
    let mut fight_result_message = String::new();

    let max_initiative = 2f32.sqrt();

    let (linitiative, _) = get_initiative_values(lentity, rentity);

    let init_gen: f32 = thread_rng().gen::<f32>() * max_initiative;

    // lentity has initiative
    if init_gen < linitiative {
        fight_result_message
            .push_str(format!("{} tries to hit {}.\n", lentity.name, rentity.name).as_str());
        // Hit or miss
        if thread_rng().gen::<f32>()
            < get_hit_chance(
                lentity.get_effective_attack(),
                rentity.get_effective_defense(),
            ) {
            let dmg = get_damage(lentity.attack, rentity.defense);
            fight_result_message
                .push_str(format!("They hit for {} damage!\n", dmg.floor()).as_str());
            rentity.health = (rentity.health - dmg as i16).min(0i16);
            if rentity.health == 0 {
                rentity.alive = false;
                fight_result_message.push_str(format!("{} has fallen!\n", rentity.name).as_str());
            }
        } else {
            fight_result_message.push_str(format!("They miss!\n").as_str());
        }

        if rentity.alive {
            fight_result_message
                .push_str(format!("{} attempts to strike back!\n", rentity.name).as_str());
            if thread_rng().gen::<f32>()
                < get_hit_chance(
                    rentity.get_effective_attack(),
                    lentity.get_effective_defense(),
                ) {
                let dmg = get_damage(rentity.attack, lentity.defense);
                fight_result_message
                    .push_str(format!("They hit for {} damage!\n", dmg.floor()).as_str());
                lentity.health = (lentity.health - dmg as i16).min(0i16);
                if lentity.health == 0 {
                    lentity.alive = false;
                    fight_result_message
                        .push_str(format!("{} has fallen!\n", lentity.name).as_str());
                }
            } else {
                fight_result_message.push_str(format!("They miss!\n").as_str());
            }
        }
    } else {
        fight_result_message
            .push_str(format!("{} tries to hit {}.\n", rentity.name, lentity.name).as_str());
        if thread_rng().gen::<f32>()
            < get_hit_chance(
                rentity.get_effective_attack(),
                lentity.get_effective_defense(),
            ) {
            let dmg = get_damage(rentity.attack, lentity.defense);
            fight_result_message
                .push_str(format!("They hit for {} damage!\n", dmg.floor()).as_str());
            lentity.health = (lentity.health - dmg as i16).min(0i16);
            if lentity.health == 0 {
                lentity.alive = false;
                fight_result_message.push_str(format!("{} has fallen!\n", lentity.name).as_str());
            }
        } else {
            fight_result_message.push_str(format!("They miss!\n").as_str());
        }

        if lentity.alive {
            fight_result_message
                .push_str(format!("{} attempts to strike back!\n", lentity.name).as_str());
            if thread_rng().gen::<f32>()
                < get_hit_chance(
                    lentity.get_effective_attack(),
                    rentity.get_effective_defense(),
                ) {
                let dmg = get_damage(lentity.attack, rentity.defense);
                fight_result_message
                    .push_str(format!("They hit for {} damage!\n", dmg.floor()).as_str());
                rentity.health = (rentity.health - dmg as i16).min(0i16);
                if rentity.health == 0 {
                    rentity.alive = false;
                    fight_result_message
                        .push_str(format!("{} has fallen!\n", rentity.name).as_str());
                }
            } else {
                fight_result_message.push_str(format!("They miss!\n").as_str());
            }
        }
    }

    if lentity.alive {
        let org_health = lentity.health;
        lentity.regen();
        let amount = lentity.health - org_health;
        if amount > 0 {
            fight_result_message
                .push_str(format!("{} regenerated {} health.\n", lentity.name, amount).as_str());
        }
    }

    if rentity.alive {
        let org_health = rentity.health;
        rentity.regen();
        let amount = rentity.health - org_health;
        if amount > 0 {
            fight_result_message
                .push_str(format!("{} regenerated {} health.\n", rentity.name, amount).as_str());
        }
    }

    fight_result_message
}
