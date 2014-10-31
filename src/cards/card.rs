use cards::suit;
use cards::value;

pub trait Card {
    fn suit(&self) -> &suit::Suit;
    fn value(&self) -> &value::Value;
}

// Can I pass a ref?  Or pointer?
pub struct CardImpl {
    pub v: value::ValueImpl,
    pub s: suit::SuitImpl,
}

impl Card for CardImpl {
    fn suit(&self) -> &suit::Suit {
        return &self.s;
    }
    fn value(&self) -> &value::Value {
        return &self.v;
    }
}

impl CardImpl {
    pub fn new(v: value::ValueImpl, s: suit::SuitImpl) -> CardImpl {
        return CardImpl {
            v: v,
            s: s,
        };
    }
}
