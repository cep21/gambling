use std::fmt;
#[derive(PartialEq, Copy, PartialOrd, Clone, Ord, Eq)]
pub enum BJAction {
    HIT,
    STAND,
    DOUBLE,
    SPLIT,
    SURRENDER,
}

impl fmt::Show for BJAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BJAction::HIT => "HIT".fmt(f),
            BJAction::STAND => "STD".fmt(f),
            BJAction::DOUBLE => "DBL".fmt(f),
            BJAction::SPLIT => "SPT".fmt(f),
            BJAction::SURRENDER => "SUR".fmt(f)
        }
    }
}
