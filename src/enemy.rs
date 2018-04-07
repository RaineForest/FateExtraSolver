
use action::Action;
use action_count::ActionCount;

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

impl fmt::Debug for Enemy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {:?}]", self.name, self.action_strings)
    }
}

impl Enemy {
    pub fn new(n: String, a: Vec<Vec<Action>>) -> Enemy {
        Enemy {
            name: n,
            action_strings: a,
        }
    }

    #[allow(dead_code)]
    pub fn get_patterns(&self) -> Vec<&Vec<Action>> {
        self.action_strings.iter().map(|x| x).collect()
    }

    pub fn matching_strings(&self, actions: &Vec<Action>) -> Vec<&Vec<Action>> {
        self.action_strings.iter().filter(|x| {
            x.iter().zip(actions.iter()).fold(true, |acc, (y, z)| { acc && (y == z) })
        }).collect()
    }

    #[allow(dead_code)]
    pub fn print_patterns(&self) {
        self.action_strings.iter().map(|x| {
            x.iter().map(|y| {
                print!("{}\t", y);
            }).collect::<Vec<_>>();
            println!();
        }).collect::<Vec<_>>();
    }

    pub fn count_actions(patterns: &Vec<&Vec<Action>>) -> Vec<ActionCount> {
        let init = patterns[0].iter().map(|e| {
            ActionCount::convert_single(e)
        }).collect::<Vec<ActionCount>>();

        let rest = &patterns[1..];
        rest.iter().fold(init, |accum, pattern| {
            accum.iter().zip(pattern.iter()).map(|(a, p)| {
                a + &ActionCount::convert_single(p)
            }).collect()
        })
    }

    pub fn trump_string(counts: &Vec<ActionCount>, strat: &Fn(&ActionCount) -> Action) -> Vec<Action> {
        counts.iter().map(strat).collect()
    }
}
