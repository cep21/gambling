use cards::suit::Suit;
use cards::value::Value;
use std::fmt;

// Can I pass a ref?  Or pointer?
#[derive(Clone, Copy)]
pub struct Card {
    v: Value,
    s: Suit,
}

impl fmt::Show for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.v.char(), self.s.char())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        self.v == other.v && self.s == other.s
    }
}

impl Card {
    pub fn suit(&self) -> &Suit {
        return &self.s;
    }
    pub fn value(&self) -> &Value {
        return &self.v;
    }
    pub fn new(v: Value, s: Suit) -> Card {
        return Card {
            v: v,
            s: s,
        };
    }
}
