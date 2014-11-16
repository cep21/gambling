use cards::suit::Suit;
use cards::value::Value;

// Can I pass a ref?  Or pointer?
pub struct Card<'a> {
    v: &'a Value,
    s: &'a Suit,
}

impl <'a>Card<'a> {
    pub fn suit(&self) -> &'a Suit {
        return self.s;
    }
    pub fn value(&self) -> &'a Value {
        return self.v;
    }
    pub fn new(v: &'a Value, s: &'a Suit) -> Card<'a> {
        return Card {
            v: v,
            s: s,
        };
    }
}
