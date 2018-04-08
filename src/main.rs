
#[macro_use] extern crate lazy_static;
extern crate regex;

mod action;
mod action_count;
mod enemy;
mod enemy_builder;

use action::Action;
use action_count::ActionCount;
use enemy::Enemy;

use std::io::{self, Write};

fn print_pattern(pattern: &Vec<Action>) {
    for element in pattern.iter() {
        print!("{}\t", element);
    }
    print!("\n");
}

#[allow(dead_code)]
fn print_patterns(patterns: &Vec<&Vec<Action>>) {
    patterns.iter().map(|x| print_pattern(&x)).collect::<Vec<_>>();
}

fn get_counter(e: &Enemy, known: &Vec<Action>, threshold: f32) -> Vec<Action> {
    Enemy::trump_string(
        &Enemy::count_actions(&e.matching_strings(&known)),
        &*ActionCount::get_safe_strat(threshold)
    )
}

fn main() {
    let enemies = enemy_builder::enemy_builder(&String::from("enemies.txt")).expect("Error in enemy builder");

    let mut line_in = String::from("");
    let mut selected_enemy = &Enemy::new(String::from(""), vec![]);
    let mut threshold = 0.1f32;
    loop {
        line_in.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line_in).expect("Could not read input");
        let tokens = line_in.as_str().trim().split_whitespace().collect::<Vec<_>>();

        if tokens.len() == 0 {
            continue;
        }

        match tokens[0] {
            "select" => {
                if tokens.len() != 2 {
                    eprintln!("Improper arguments: select <Enemy name>");
                } else {
                    match enemies.get(tokens[1]) {
                        Some(x) => selected_enemy = &x,
                        None => eprintln!("Invalid enemy name \"{}\"", tokens[1]),
                    }
                }
            },
            "solve" => {
                if tokens.len() != 7 {
                    eprintln!("Improper arguments: solve _ _ _ _ _ _");
                }
                else if selected_enemy.get_name() == "" {
                    eprintln!("No enemy selected to solve for.");
                }
                else {
                    let p = tokens[1..].iter().map(|&x| Action::deserialize(&String::from(x))).collect::<Vec<_>>();
                    print_pattern(&get_counter(&selected_enemy, &p, threshold));
                }
            }
            "threshold" => {
                if tokens.len() != 2 {
                    eprintln!("Improper arguments: threshold <0-100>");
                } else {
                    threshold = tokens[1].to_string().parse::<f32>().unwrap() / 100.0; // TODO: proper error handling
                }
            }
            "list" => {
                enemies.iter().map(|(k,_)| {
                    println!("{}", k);
                }).collect::<Vec<_>>();
            }
            "exit" => {
                break;
            }
            "help" => {
                println!("Commands available:\n \
                          select <Enemy name> => future commands will use this enemy as a reference\n \
                          solve <[A|G|B|S|_] * 6, space sep> => solve this pattern\n \
                          threshold <[0-100]> => set damage potential, doesn't do anything over 50%\n \
                          list => list all enemies loaded\n \
                          help => print this message!\n \
                          exit => close the application");
            }
            _ => {
                eprintln!("Invalid command");
            }
        }
    }
}
