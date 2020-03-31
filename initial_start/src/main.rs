use math::round;
use rand::Rng;
use std::io;

struct GameState {
    health: i32,
    max_health: i32,
    magic: i32,
    stage: i32,
}

fn main() {
    let mut gs = GameState {
        health: 30,
        max_health: 30,
        magic: 30,
        stage: 1,
    };
    loop {
        if gs.health < 1 {
            println!("You are dead!");
            break;
        }

        if gs.stage >= 10 {
            println!("You conquered the dungeon!!");
            break;
        }

        if gs.health < gs.max_health {
            if gs.magic >= 5 {
                println!("Use healing magic? Type 'y' for yes");
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
                    stat_update(gs.health, gs.magic);
                }
            } else {
                println!("Not enough magic to heal!")
            }
        }
        if gs.stage == 1 {
            println!("Welcome to the dungeon! There are 10 levels, will you survive?")
        }
        confirmation();
        // 75% and rising chance to encounter a goblin...
        if get_percentage() > 25 + 2 * gs.stage {
            let combat_outcome: (i32, i32) = enter_combat(gs.stage, gs.health, gs.magic);
            gs.health = combat_outcome.0;
            gs.magic = combat_outcome.1;
        } else {
            println!("Nothing encountered in stage {}", gs.stage)
        }
        stat_update(gs.health, gs.magic);
        gs.stage += gs.stage;
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
    current_magic: i32,
    enemy_health: i32,
    round: i32,
}

struct Creature {
    cre_health: i32,
    cre_low_att_ran: i32,
    cre_high_att_ran: i32,
}

fn enter_combat(stage: i32, health: i32, magic: i32) -> (i32, i32) {
    // set range but round down
    let half_round = stage / 2;
    let cur_mon_att_bonus = round::floor(half_round, 0);
    println!("this thing {}", cur_mon_att_bonus);
    let mut creature = Creature {
        cre_health: 5 + stage,
        cre_low_att_ran: 2 + cur_mon_att_bonus,
        cre_high_att_ran: 4 + cur_mon_att_bonus,
    };

    let mut cs = CombatState {
        hero_turn: true,
        current_health: health,
        current_magic: magic,
        enemy_health: 10,
        round: 1,
    };
    println!("At stage {}, you encounter a goblin!", stage);
    if get_percentage() > 50 {
        println!("...The goblin attacks first...");
        cs.hero_turn = false
    } else {
        println!("...You attack first!...");
        cs.hero_turn = true
    }
    confirmation();
    loop {
        println!("*Round {}*", cs.round);
        if cs.hero_turn {
            cs.enemy_health = hero_attack(creature.cre_health);
            if cs.enemy_health < 1 {
                println!("You defeated the goblin!");
                stat_update(cs.current_health, cs.current_magic);
                confirmation();
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
        cs.round += cs.round;
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
    println!("You attack with your sword! You do {} damage", damage);
    let new_health: i32 = health - damage;
    new_health
}

fn creature_attack(health: i32, cre_low_rng: i32, cre_high_rng: i32) -> i32 {
    let damage = rand::thread_rng().gen_range(cre_low_rng, cre_high_rng);
    println!("It attacks you and does {} damage", damage);
    let new_health: i32 = health - damage;
    new_health
}

fn stat_update(health: i32, magic: i32) {
    println!("Health: {} || magic: {}", health, magic)
}
