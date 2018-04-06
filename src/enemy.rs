
use action::Action;

use std::fmt;

pub struct Enemy {
    name: String,
    action_strings: Vec<Vec<Action>>,
}

impl fmt::Display for Enemy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Enemy {
    pub fn new(n: String, a: Vec<Vec<Action>>) -> Enemy {
        Enemy {
            name: n,
            action_strings: a,
        }
    }

    pub fn get_patterns(&self) -> Vec<&Vec<Action>> {
        self.action_strings.iter().map(|x| x).collect()
    }

    pub fn matching_strings(&self, actions: &Vec<Action>) -> Vec<&Vec<Action>> {
        self.action_strings.iter().filter(|x| {
            x.iter().zip(actions.iter()).fold(true, |acc, (y, z)| { acc && (y == z) })
        }).collect()
    }

    pub fn print_patterns(&self) {
        self.action_strings.iter().map(|x| {
            x.iter().map(|y| {
                print!("{}\t", y);
            }).collect::<Vec<_>>();
            println!();
        }).collect::<Vec<_>>();
    }

    fn action_tuple(x: &Action) -> (u32, u32, u32, u32) {
        match x {
            &Action::Attack  => (1,0,0,0),
            &Action::Guard   => (0,1,0,0),
            &Action::Break   => (0,0,1,0),
            &Action::Special => (0,0,0,1),
            _                => (0,0,0,0),
        }
    }

    pub fn tuple_max(tup: &(u32,u32,u32,u32)) -> Action {
        match tup {
            &(a, b, c, d) if a > b && a > c && a > d => Action::Attack,
            &(a, b, c, d) if b > a && b > c && b > d => Action::Guard,
            &(a, b, c, d) if c > a && c > b && c > d => Action::Break,
            &(a, b, c, d) if d > a && d > b && d > c => Action::Special,
            _ => Action::Special,
        }
    }

    pub fn count_actions(patterns: &Vec<&Vec<Action>>) -> Vec<(u32,u32,u32,u32)> {
        let init = patterns[0].iter().map(|e| {
            Enemy::action_tuple(e)
        }).collect::<Vec<(u32,u32,u32,u32)>>();

        let rest = &patterns[1..];
        rest.iter().fold(init, |accum, pattern| {
            accum.iter().zip(pattern.iter()).map(|(&(a1,a2,a3,a4), p)| {
                let (p1,p2,p3,p4) = Enemy::action_tuple(p);
                (a1+p1,a2+p2,a3+p3,a4+p4)
            }).collect()
        })
    }

    pub fn trump_string(counts: &Vec<(u32,u32,u32,u32)>, strat: &Fn(&(u32,u32,u32,u32)) -> Action) -> Vec<Action> {
        counts.iter().map(strat).collect()
    }
}
