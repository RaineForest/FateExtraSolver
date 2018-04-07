
use action::Action;
use enemy::Enemy;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::prelude::*;

pub fn read_pattern(r: &Regex, line: &String, start: usize) -> Vec<Action> {
    let mut acts = vec![];
    for cap in r.captures_iter(line) {
        for a in start..cap.len() {
            acts.push(Action::deserialize(&cap[a].to_string()));
        }
    }
    acts
}

pub fn enemy_builder(filename: &String) -> Result<HashMap<String, Enemy>, Error> {
    let mut enemy_map = HashMap::new();

    lazy_static!{
        static ref RE_NAME: Regex = Regex::new(r"^(.+):$").unwrap();
        static ref RE_ACTIONS: Regex = Regex::new(r"^([AGBS_])\s+([AGBS_])\s+([AGBS_])\s+([AGBS_])\s+([AGBS_])\s+([AGBS_])$").unwrap();
    }

    let mut curr_name = String::from("");
    let mut curr_actions: Vec<Vec<Action>> = vec![];
    let mut first = true;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    for line in reader.lines().map(|l| String::from(l.unwrap().as_str().trim())) {
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
            curr_actions.push(read_pattern(&RE_ACTIONS, &line, 1));
        }
    }

    enemy_map.insert(curr_name.clone(), Enemy::new(
        curr_name.clone(),
        curr_actions.clone(),
    ));

    Result::Ok(enemy_map)
}
