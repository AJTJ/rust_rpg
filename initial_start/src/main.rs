use rand::Rng;
use std::io;

// let stages: [String: 1] = []

fn main() {
    let mut health: i32 = 30;
    let max_health: i32 = 30;
    let magic: i32 = 30;
    let mut stage: u32 = 1;
    let in_combat: bool = false;
    loop {
        if health < 1 {
            println!("You are dead!");
            break;
        }

        if health < max_health {
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
                stat_update(health, magic);
            }
        }
        confirmation();
        // 50% chance to encounter a creature...
        if get_percentage() > 50 {
            enter_combat(stage, health, magic);
        } else {
            println!("Nothing encountered here...")
        }
        stat_update(health, magic);
        stage = stage + 1;
    }
}

fn confirmation() {
    println!("Press Enter to continue");
    let mut any_input = String::new();
    io::stdin()
        .read_line(&mut any_input)
        .expect("Click Enter Error");
}

fn enter_combat(stage: u32, health: i32, magic: i32) -> (i32, i32) {
    loop {
        let mut round: i32 = 1;
        let hero_initiative;
        let mut current_health: i32 = health;
        let mut enemy_health: i32 = 10;
        println!("You encounter a goblin!");
        confirmation();
        // get initiative
        if get_percentage() > 50 {
            println!("The goblin attacks first");
            hero_initiative = false
        } else {
            println!("You attack first!");
            hero_initiative = true
        }

        loop {
            if hero_initiative == true {
                enemy_health = hero_attack(enemy_health);
                if enemy_health < 1 {
                    println!("You defeated the goblin!")
                    break;
                }
                current_health = creature_attack(current_health);
                if current_health < 1 {
                    break;
                };
                round = round + 1;
            }
        }
        let final_health: i32 = current_health;
        (final_health, magic)
    }
}

fn get_percentage() -> i32 {
    rand::thread_rng().gen_range(1, 100)
}

fn check_yes_no(response: &str) -> bool {
    println!("in check {}", response);
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
    println!("It attacks you and does {} damage", damage);
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
