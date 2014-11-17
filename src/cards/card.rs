use cards::suit::Suit;
use cards::value::Value;

// Can I pass a ref?  Or pointer?
pub struct Card {
    v: Value,
    s: Suit,
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
