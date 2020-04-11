use rand::Rng;
use std::io;

struct GameState {
    health: i32,
    max_health: i32,
    magic: i32,
    max_magic: i32,
    stage: i32,
}

fn main() {
    let mut gs = GameState {
        health: 30,
        max_health: 30,
        magic: 30,
        max_magic: 30,
        stage: 1,
    };
    loop {
        if gs.health < 1 {
            println!("You are dead!");
            break;
        }

        println!("stage: {}", gs.stage);

        if gs.stage >= 10 {
            println!("You conquered the dungeon!!");
            break;
        }

        if gs.health < gs.max_health {
            if gs.magic >= 5 {
                println!("Use healing magic? Type 'y' for yes (heals 1-4 for 5 magic)");
                let mut response = String::new();
                io::stdin()
                    .read_line(&mut response)
                    .expect("Response Error");
                if check_yes_no(&response) {
                    let current_heal: i32 = self_heal();
                    println!("You heal {}!", current_heal);
                    gs.health = current_heal + gs.health;
                    if gs.health > gs.max_health {
                        gs.health = gs.max_health;
                    }
                    gs.magic = gs.magic - 5;
                    stat_update(gs.health, gs.max_health, gs.magic, gs.max_magic);
                }
            } else {
                println!("Not enough magic to heal!")
            }
        }
        if gs.stage == 1 {
            println!("Welcome to the dungeon! There are 10 levels, will you survive?")
        }
        confirmation();
        // 75% and rising chance to encounter an enemy...
        if get_percentage() > 25 + 2 * gs.stage {
            let combat_outcome: (i32, i32) = enter_combat(gs.stage, gs.health, gs.magic);
            gs.health = combat_outcome.0;
            gs.magic = combat_outcome.1;
        } else {
            println!("Nothing encountered in stage {}", gs.stage)
        }
        stat_update(gs.health, gs.max_health, gs.magic, gs.max_magic);
        gs.stage += 1;
    }
}

fn confirmation() {
    println!("--- Press Enter to continue ---");
    let mut any_input = String::new();
    io::stdin()
        .read_line(&mut any_input)
        .expect("Click Enter Error");
}

struct CombatState {
    hero_turn: bool,
    current_health: i32,
    // current_magic: i32,
    enemy_health: i32,
    round: i32,
}

struct Creature<'a> {
    cre_health: i32,
    cre_low_att_ran: i32,
    cre_high_att_ran: i32,
    cre_name: &'a str,
}

fn enter_combat(stage: i32, health: i32, magic: i32) -> (i32, i32) {
    let quarter_round = stage / 4;
    let third_round = stage / 3;
    let set_creature_name;
    if stage > 7 {
        set_creature_name = "Master Goblin";
    } else if stage > 4 {
        set_creature_name = "Big Goblin";
    } else {
        set_creature_name = "Goblin"
    };

    let creature = Creature {
        cre_health: 5 + stage,
        cre_low_att_ran: 2 + quarter_round,
        cre_high_att_ran: 4 + third_round,
        cre_name: set_creature_name,
    };

    let mut cs = CombatState {
        hero_turn: true,
        current_health: health,
        // current_magic: magic,
        enemy_health: creature.cre_health,
        round: 1,
    };
    println!("At stage {}, you encounter a {}!", stage, creature.cre_name);
    println!(
        "It has {} hit points and does {} to {} damage!",
        creature.cre_health, creature.cre_low_att_ran, creature.cre_high_att_ran
    );
    if get_percentage() > 50 {
        println!("...The {} attacks first...", creature.cre_name);
        cs.hero_turn = false
    } else {
        println!("...You attack first!...");
        cs.hero_turn = true
    }
    confirmation();
    loop {
        println!("*Round {}*", cs.round);
        if cs.hero_turn {
            cs.enemy_health = hero_attack(cs.enemy_health);
            if cs.enemy_health < 1 {
                println!("You defeated the {}!", creature.cre_name);
                // stat_update(cs.current_health, cs.current_magic);
                // confirmation();
                break;
            }
        } else {
            cs.current_health = creature_attack(
                cs.current_health,
                creature.cre_low_att_ran,
                creature.cre_high_att_ran,
            );
            if cs.current_health < 1 {
                break;
            };
        };
        cs.hero_turn = !cs.hero_turn;
        cs.round += 1;
    }
    let final_health: i32 = cs.current_health;
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
    println!("- You attack with your sword! You do {} damage", damage);
    let new_health: i32 = health - damage;
    new_health
}

fn creature_attack(health: i32, cre_low_rng: i32, cre_high_rng: i32) -> i32 {
    let damage = rand::thread_rng().gen_range(cre_low_rng, cre_high_rng);
    println!("- It attacks you and does {} damage", damage);
    let new_health: i32 = health - damage;
    new_health
}

fn stat_update(health: i32, max_health: i32, magic: i32, max_magic: i32) {
    println!(
        "Health: {}/{} || magic: {}/{}",
        health, max_health, magic, max_magic
    )
}
