
use action::Action;
use enemy::Enemy;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;

fn action_deserialize(s: &String) -> Action {
    match s.as_ref() {
        "A" => Action::Attack,
        "B" => Action::Break,
        "G" => Action::Guard,
        "S" => Action::Special,
        _ => panic!("Deserialize bad value!"),
    }
}

pub fn enemy_builder(filename: &String) -> Result<HashMap<String, Enemy>, Error> {
    let mut enemy_map = HashMap::new();

    lazy_static!{
        static ref RE_NAME: Regex = Regex::new(r"^(.+):\s*$").unwrap();
        static ref RE_ACTIONS: Regex = Regex::new(r"^([AGBS_])\s+([AGBS_])\s+([AGBS_])\s+([AGBS_])\s+([AGBS_])\s+([AGBS_])\s*$").unwrap();
    }

    let mut curr_name = String::from("");
    let mut curr_actions: Vec<Vec<Action>> = vec![];
    let mut first = true;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines().map(|l| l.unwrap()) {
        if RE_NAME.is_match(&line) {
            if !first {
                enemy_map.insert(curr_name.clone(), Enemy::new(
                    curr_name.clone(),
                    curr_actions.clone(),
                ));
            }
            first = false;

            for cap in RE_NAME.captures_iter(&line) {
                curr_name = cap[1].to_string().clone();
                curr_actions.clear();
            }
        }
        else if RE_ACTIONS.is_match(&line) {
            let mut acts = vec![];
            for cap in RE_ACTIONS.captures_iter(&line) {
                for a in 1..cap.len() {
                    acts.push(action_deserialize(&cap[a].to_string()));
                }
            }
            curr_actions.push(acts);
        }
    }

    enemy_map.insert(curr_name.clone(), Enemy::new(
        curr_name.clone(),
        curr_actions.clone(),
    ));

    Result::Ok(enemy_map)
}
