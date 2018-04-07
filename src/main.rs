
#[macro_use] extern crate lazy_static;
extern crate regex;

mod action;
mod action_count;
mod enemy;
mod enemy_builder;

use action::Action;
use Action::*;
use action_count::ActionCount;
use enemy::Enemy;

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

    let e = &enemies["Inspire"];
    println!("{}'s patterns:", e);
    e.print_patterns();
    println!("Counts: {:?}", Enemy::count_actions(&e.get_patterns()));

    let known = vec![Any, Guard, Any, Any, Any, Any];
    println!("Known actions:");
    print_pattern(&known);

    let threshold1 = 0.1f32; // 10%
    let threshold2 = 0.4f32; // 40%

    println!("Safe trumps: ({}%)", (threshold1 * 100.0f32).round() as i32);
    print_pattern(&get_counter(&e, &known, threshold1));
    println!("Safe trumps: ({}%)", (threshold2 * 100.0f32).round() as i32);
    print_pattern(&get_counter(&e, &known, threshold2));
}
