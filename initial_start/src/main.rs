use rand::Rng;
use std::io;

//NOTES
/*
21:49 < Mutabah> Quick check:
21:49 < Mutabah> - I suggest using a `struct` to contain the game state variables
21:49 < Mutabah> - abstract out the yes/no query
21:50 < AJTJ> game state variables like health etc?
21:50 < Mutabah> - use `+=` instead of `stage = stage + 1;`
21:50 < Mutabah> (yes)
21:50 < AJTJ> should I be using more mutable vars?
21:52 < Mutabah> - You could remove duplication in `enter_combat` by keepting track of whose turn it is (initialise that where you currently set `hero_initiative`)
21:53 < GreenJello> AJTJ, with a game state struct, you might take a mutable reference to that instead
21:54 < Arnavion> Eh, for a game that's one function, using local vars for the state instead of a struct isn't a big deal
21:54 < AJTJ> Arnavion: that's a good point, but it's good to think of scaling
21:54 < AJTJ> cause it will
21:57 < GreenJello> you could abstract a lot of this logic with structs, though, e.g. https://gist.github.com/brigand/2573131a7045d2617d80a4fab2b088db
21:57 < Arnavion> I'd more prioritize fixing the fact that the two branches inside the loop in enter_combat() are mirror images of each other
21:58 < AJTJ> Arnavion: yea, I wasn't sure how to resolve that. I'm super new to rust.
21:58 < Arnavion> if hero_initialive { hero_attack(); } loop { creature_attack(); hero_attack(); }
22:00 < Mutabah> or `let mut hero_turn = hero_initiative; loop { if hero_turn { ... } else { ... }; half_rounds += 1; hero_turn = !hero_turn; }`
22:01 < Arnavion> Yeah, that would avoid the duplication of hero_attack() too
22:01 < Mutabah> and allow common code after each half-round (e.g. if attacking a certain monster can lead to self-damage)
*/

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
