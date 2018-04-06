
mod action;
mod enemy;

use action::Action;
use Action::*;
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
    let patterns: [Vec<Action>; 3] = [
        vec![Action::Break, Action::Break, Action::Attack, Action::Break, Action::Break, Action::Attack],
        vec![Action::Attack, Action::Attack, Action::Guard, Action::Guard, Action::Attack, Action::Attack],
        vec![Action::Break, Action::Break, Action::Break, Action::Break, Action::Break, Action::Break]
    ];

    for pattern in patterns.iter() {
        print!("Pattern: ");
        print_pattern(&pattern);

        let weaks = pattern.iter().map(|x| {x.what_weak()}).collect::<Vec<_>>();
        print!("Weak:    ");
        print_pattern(&weaks);

        let trumps = pattern.iter().map(|x| {x.what_trumps()}).collect::<Vec<_>>();
        print!("Trump:   ");
        print_pattern(&trumps);

        print!("\n");
    }

    let x = Action::Any;
    let y = Action::Attack;
    let z = Action::Attack;
    println!("{}", x == y);
    println!("{}", y == x);
    println!("{}", y == z);

    println!();

    let e = Enemy::new(
        String::from("Inspire"),
        vec![
            vec![Attack, Guard, Guard, Guard, Attack, Guard],
            vec![Guard, Guard, Guard, Guard, Guard, Guard],
            vec![Guard, Guard, Break, Break, Guard, Guard],
        ]
    );

    let known = vec![Any, Guard, Any, Guard, Any, Any];

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

    let aggressive_strat = |tup: &(u32,u32,u32,u32)| -> Action {
        Enemy::tuple_max(&tup).what_trumps()
    };

    println!("Aggressive trumps:");
    print_pattern(&Enemy::trump_string(&counts, &aggressive_strat));
}
