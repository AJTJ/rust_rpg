use rand::Rng;
use std::io;

// let stages: [String: 1] = []

fn main() {
    let mut health: i32 = 30;
    let magic: i32 = 30;
    let stage: u32 = 1;

    let mut any_input = String::new();
    loop {
        if health < 1 {
            println!("You are dead!");
            break;
        }
        println!("You are at stage {}, press Enter to continue.", stage);
        io::stdin()
            .read_line(&mut any_input)
            .expect("Incorrect input");
        health = creature_attack(stage, health);
        stat_update(health, magic);
    }
}

fn creature_attack(stage: u32, health: i32) -> i32 {
    let damage = rand::thread_rng().gen_range(1, 4);
    println!(
        "At stage {}, you encountered a goblin! It attacks you and does {} damage",
        stage, damage
    );
    let new_health: i32 = health - damage;
    new_health
}

fn stat_update(health: i32, magic: i32) {
    println!("Health: {} || magic: {}", health, magic)
}
