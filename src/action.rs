
use std::fmt;

#[derive(Clone)]
pub enum Action {
    Attack,
    Guard,
    Break,
    Special,
    Any,
}

impl Action {
    pub fn what_weak(&self) -> Action {
        match *self {
            Action::Attack  => Action::Break,
            Action::Guard   => Action::Attack,
            Action::Break   => Action::Guard,
            Action::Special => Action::Special,
            Action::Any     => Action::Any,
        }
    }

    pub fn what_trumps(&self) -> Action {
        match *self {
            Action::Attack  => Action::Guard,
            Action::Guard   => Action::Break,
            Action::Break   => Action::Attack,
            Action::Special => Action::Special,
            Action::Any     => Action::Any,
        }
    }

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Action::Attack  => "Attack",
            Action::Guard   => "Guard",
            Action::Break   => "Break",
            Action::Special => "Special",
            Action::Any     => "_______",
        })
    }

    pub fn deserialize(s: &String) -> Action {
        match s.as_ref() {
            "A" => Action::Attack,
            "B" => Action::Break,
            "G" => Action::Guard,
            "S" => Action::Special,
            "_" => Action::Any,
            _ => panic!("Deserialize bad value!"),
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Action) -> bool {
        use Action::*;
        match (self, other) {
            (&Any, _) => true,
            (_, &Any) => true,
            (&Attack, &Attack) => true,
            (&Guard, &Guard) => true,
            (&Break, &Break) => true,
            (&Special, &Special) => true,
            _ => false,           
        }        
    }
}
