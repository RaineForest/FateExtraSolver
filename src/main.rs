
mod action;
mod action_count;
mod enemy;

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

fn print_patterns(patterns: &Vec<&Vec<Action>>) {
    patterns.iter().map(|x| print_pattern(&x)).collect::<Vec<_>>();
}

fn main() {
    let e = Enemy::new(
        String::from("Inspire"),
        vec![
            vec![Attack, Guard, Guard, Guard, Attack, Guard],
            vec![Guard, Guard, Guard, Guard, Guard, Guard],
            vec![Guard, Guard, Break, Break, Break, Guard],
        ]
    );

    let known = vec![Any, Guard, Any, Any, Any, Any];

    let matching = e.matching_strings(&known);
    println!("{}'s patterns:", e);
    e.print_patterns();
    println!("Counts: {:?}", Enemy::count_actions(&e.get_patterns()));

    println!("Known actions:");
    print_pattern(&known);

    println!("{}'s matching patterns:", e);
    print_patterns(&matching);
    let counts = Enemy::count_actions(&matching);
    println!("Counts: {:?}", &counts);

    let aggressive_strat = |tup: &ActionCount| -> Action {
        tup.get_max().what_trumps()
    };

    let threshold1 = 0.1f32; // 10%
    let threshold2 = 0.4f32; // 40%

    println!("Aggressive trumps:");
    print_pattern(&Enemy::trump_string(&counts, &aggressive_strat));
    println!("Safe trumps: ({}%)", (threshold1 * 100.0f32).round() as i32);
    print_pattern(&Enemy::trump_string(&counts, &*ActionCount::get_safe_strat(threshold1)));
    println!("Safe trumps: ({}%)", (threshold2 * 100.0f32).round() as i32);
    print_pattern(&Enemy::trump_string(&counts, &*ActionCount::get_safe_strat(threshold2)));
}
