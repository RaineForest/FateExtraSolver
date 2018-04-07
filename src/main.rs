
#[macro_use] extern crate lazy_static;
extern crate regex;

mod action;
mod action_count;
mod enemy;
mod enemy_builder;

use action::Action;
use action_count::ActionCount;
use enemy::Enemy;

use regex::Regex;
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
    lazy_static!{
        static ref RE_SELECT: Regex = Regex::new(r"^(select)\s+(.+)$").unwrap();
        static ref RE_PATTERN: Regex = Regex::new(r"^(solve)\s+([AGBS_])\s+([AGBS_])\s+([AGBS_])\s+([AGBS_])\s+([AGBS_])\s+([AGBS_])$").unwrap();
        static ref RE_THRESHOLD: Regex = Regex::new(r"^(threshold)\s+(\d+(?:\.\d+)?)$").unwrap();
    }

    let enemies = enemy_builder::enemy_builder(&String::from("enemies.txt")).expect("Error in enemy builder");

    let mut line_in = String::from("");
    let mut selected_enemy = &Enemy::new(String::from(""), vec![]);
    let mut threshold = 0.1f32;
    loop {
        line_in.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line_in).expect("Could not read input");
        line_in = String::from(line_in.as_str().trim());
        if RE_SELECT.is_match(&line_in) {
            for cap in RE_SELECT.captures_iter(&line_in) {
                selected_enemy = &enemies[&cap[2].to_string()]; // TODO: proper error handling
            }
        }
        else if RE_PATTERN.is_match(&line_in) {
            let p = enemy_builder::read_pattern(&RE_PATTERN, &line_in, 2);
            print_pattern(&get_counter(&selected_enemy, &p, threshold));
        }
        else if RE_THRESHOLD.is_match(&line_in) {
            for cap in RE_THRESHOLD.captures_iter(&line_in) {
                threshold = cap[2].to_string().parse::<f32>().unwrap() / 100.0; // TODO: proper error handling
            }
        }
        else if line_in == "exit" {
            break;
        }
        else if line_in == "help" {
            println!("Commands available:\n \
                      select <Enemy name> => future commands will use this enemy as a reference\n \
                      solve <[A|G|B|S|_] * 6, space sep> => solve this pattern\n \
                      threshold <[0-100]> => set damage potential, doesn't do anything over 50%\n \
                      help => print this message!\n \
                      exit => close the application");
        }
        else {
            println!("Invalid command");
        }
    }
}
