
use action::Action;

use std::fmt;
use std::ops::Add;

pub struct ActionCount {
    attacks: u32,
    guards: u32,
    breaks: u32,
    specials: u32
}

impl fmt::Display for ActionCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for ActionCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl<'a> Add for &'a ActionCount {
    type Output = ActionCount;

    fn add(self, other: &'a ActionCount) -> ActionCount {
        ActionCount {
            attacks: self.attacks + other.attacks,
            guards: self.guards + other.guards,
            breaks: self.breaks + other.breaks,
            specials: self.specials + other.specials,
        }
    }
}

impl ActionCount {
    pub fn convert_single(a: &Action) -> ActionCount {
        match a {
            &Action::Attack  => ActionCount { attacks: 1, guards: 0, breaks: 0, specials: 0, },
            &Action::Guard   => ActionCount { attacks: 0, guards: 1, breaks: 0, specials: 0, },
            &Action::Break   => ActionCount { attacks: 0, guards: 0, breaks: 1, specials: 0, },
            &Action::Special => ActionCount { attacks: 0, guards: 0, breaks: 0, specials: 1, },
            _                => ActionCount { attacks: 0, guards: 0, breaks: 0, specials: 0, },
        }
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(A:{}, G:{}, B:{}, S:{})", self.attacks, self.guards, self.breaks, self.specials)
    }

    pub fn get_count(&self, a: &Action) -> u32 {
        match a {
            &Action::Attack  => self.attacks,
            &Action::Guard   => self.guards,
            &Action::Break   => self.breaks,
            &Action::Special => self.specials,
            _                => 0,
        }
    }

    pub fn get_max(&self) -> Action {
        match self {
            _ if self.attacks > self.breaks && self.attacks > self.guards && self.attacks > self.specials => Action::Attack,
            _ if self.guards > self.attacks && self.guards > self.breaks && self.guards > self.specials => Action::Guard,
            _ if self.breaks > self.attacks && self.breaks > self.guards && self.breaks > self.specials => Action::Break,
            _ if self.specials > self.attacks && self.specials > self.guards && self.specials > self.breaks => Action::Special,
            _ => Action::Special,
        }
    }

    fn total(&self) -> u32 {
        self.attacks + self.guards + self.breaks + self.specials
    }

    fn likelihood(&self) -> [f32; 4] {
        let t = self.total() as f32;
        [
            self.attacks as f32 / t,
            self.guards as f32 / t,
            self.breaks as f32 / t,
            self.specials as f32 / t,
        ]
    }

    fn index_to_action(x: usize) -> Action {
        match x {
            0 => Action::Attack,
            1 => Action::Guard,
            2 => Action::Break,
            3 => Action::Special,
            _ => panic!("Invalid index!"),
        }
    }

    pub fn get_safe(&self, threshold: f32) -> Action {
        let l = self.likelihood();
        let mut m = 0f32;
        let mut m_index = vec![];
        for (i, &f) in l.iter().enumerate() {
            if f > m {
                m = f;
                m_index.push(i);
            }
        }

        match m_index.len() {
            /*
             * Find the trump of the max, and compare the trump's trump
             * percentage against the threshold (punish potential)
             */
            1 => {
                match m_index[0] {
                    3 => Action::Special,
                    _ => {
                        let t = (m_index[0] + 1) % 3; // trump index
                        let tt = l[(t + 1) % 3]; // get the trump's trump to see punish percent
                        if tt >= threshold { // play it safe
                            ActionCount::index_to_action(m_index[0])
                        } else { // it's ok to get aggressive
                            ActionCount::index_to_action(t)
                        }
                    }
                } 
            },
            /*
             * Find the trump of the two and use that one (unless the other one is over threshold)
             * Worst case => clang
             * Best case => got a hit
             */
            2 => {
                if m_index[0] == 3 || m_index[1] == 3 {
                    return Action::Special;
                }

                let mut t = 0;
                if ((m_index[0] + 1) % 3) == m_index[1] {
                    t = m_index[1];
                } else if ((m_index[1] + 1) % 3) == m_index[0] {
                    t = m_index[0];
                }

                if l[(t + 1) % 3] >= threshold {
                    Action::Special
                } else {
                    ActionCount::index_to_action(t)
                }
            },
            // if 3 or 4 are tied for the max, then a special is the safe play
            _ => Action::Special,
        }
    }

    pub fn get_safe_strat(threshold: f32) -> Box<Fn(&ActionCount) -> Action> {
        Box::new(move |tup: &ActionCount| -> Action {
            tup.get_safe(threshold)
        })
    }
}
