use suit;
use value;

pub trait Card {
    fn suit(&self) -> &suit::Suit;
    fn value(&self) -> &value::Value;
}

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
