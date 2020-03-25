use rand::Rng;
use std::io;

// let stages: [String: 1] = []

fn main() {
    let mut health: i32 = 30;
    let max_health: i32 = 30;
    let mut magic: i32 = 30;
    let mut stage: u32 = 1;
    loop {
        if health < 1 {
            println!("You are dead!");
            break;
        }

        if stage >= 10 {
            println!("You conquered the dungeon!!");
            break;
        }

        if health < max_health {
            if magic >= 5 {
                println!("Use healing magic? Type 'y' for yes");
                let mut response = String::new();
                io::stdin()
                    .read_line(&mut response)
                    .expect("Response Error");
                if check_yes_no(&response) {
                    let current_heal: i32 = self_heal();
                    println!("You heal {}!", current_heal);
                    health = current_heal + health;
                    if health > max_health {
                        health = max_health;
                    }
                    magic = magic - 5;
                    stat_update(health, magic);
                }
            } else {
                println!("Not enough magic to heal!")
            }
        }
        if stage == 1 {
            println!("Welcome to the dungeon! There are 10 stages, will you survive?")
        }
        confirmation();
        // 75% chance to encounter a goblin...
        if get_percentage() > 25 {
            let combat_outcome: (i32, i32) = enter_combat(stage, health, magic);
            health = combat_outcome.0;
            magic = combat_outcome.1;
        } else {
            println!("Nothing encountered in stage {}", stage)
        }
        stat_update(health, magic);
        stage = stage + 1;
    }
}

fn confirmation() {
    println!("--- Press Enter to continue ---");
    let mut any_input = String::new();
    io::stdin()
        .read_line(&mut any_input)
        .expect("Click Enter Error");
}

fn enter_combat(stage: u32, health: i32, magic: i32) -> (i32, i32) {
    let hero_initiative;
    let mut current_health: i32 = health;
    let current_magic: i32 = magic;
    let mut enemy_health: i32 = 10;
    let mut round: i32 = 1;
    println!("At stage {}, you encounter a goblin!", stage);
    if get_percentage() > 50 {
        println!("...The goblin attacks first...");
        hero_initiative = false
    } else {
        println!("...You attack first!...");
        hero_initiative = true
    }
    confirmation();
    loop {
        println!("*Round {}*", round);
        if hero_initiative == true {
            enemy_health = hero_attack(enemy_health);
            if enemy_health < 1 {
                println!("You defeated the goblin!");
                stat_update(current_health, current_magic);
                confirmation();
                break;
            }
            current_health = creature_attack(current_health);
            if current_health < 1 {
                break;
            };
        } else {
            current_health = creature_attack(current_health);
            if current_health < 1 {
                break;
            };
            enemy_health = hero_attack(enemy_health);
            if enemy_health < 1 {
                println!("You defeated the goblin!");
                stat_update(current_health, current_magic);
                confirmation();
                break;
            }
        };
        round = round + 1;
    }
    let final_health: i32 = current_health;
    (final_health, magic)
}

fn get_percentage() -> i32 {
    rand::thread_rng().gen_range(1, 100)
}

fn check_yes_no(response: &str) -> bool {
    match response.trim() {
        "y" | "yes" | "Y" | "Yes" => true,
        "n" | "no" | "N" | "No" => false,
        _ => false,
    }
}

fn self_heal() -> i32 {
    rand::thread_rng().gen_range(1, 4)
}

fn sword_damage() -> i32 {
    rand::thread_rng().gen_range(1, 9)
}

fn hero_attack(health: i32) -> i32 {
    let damage = sword_damage();
    println!("You attack with your sword! You do {} damage", damage);
    let new_health: i32 = health - damage;
    new_health
}

fn creature_attack(health: i32) -> i32 {
    let damage = rand::thread_rng().gen_range(1, 4);
    println!("It attacks you and does {} damage", damage);
    let new_health: i32 = health - damage;
    new_health
}

fn stat_update(health: i32, magic: i32) {
    println!("Health: {} || magic: {}", health, magic)
}
