use shoe;
use cards;
use suit;
use value;
use shoe::DirectShoe;
use std::rand;

pub trait SuitPicker {
    fn suit(&mut self) -> Option<&suit::Suit>;
}

struct CycleSuitPicker {
    suitIndex: uint,
}

impl SuitPicker for CycleSuitPicker {
    fn suit(&mut self) -> Option<&suit::Suit> {
        self.suitIndex += 1;
        let r:&suit::Suit = &suit::SUITS[self.suitIndex % 4];
        return Some(r);
    }
}

struct SuitCount {
    suit: suit::SuitImpl,
    counts: uint,
}

struct RandomDeckSuitPicker {
    suitCounts: Vec<SuitCount>,
}

impl SuitPicker for RandomDeckSuitPicker {
    fn suit(&mut self) -> Option<&suit::Suit> {
        if self.suitCounts.len() == 0 {
            return None;
        }
        let (suitToRet, remove_index) = {
            let suitIndex = rand::random::<uint>() % self.suitCounts.len();
            let ref mut suitToLook = self.suitCounts.get_mut(suitIndex);
            let suitToRet = suitToLook.suit;
            suitToLook.counts -= 1;
            if suitToLook.counts == 0 {
                (suitToRet, Some(suitIndex))
            } else {
                (suitToRet, None)
            }
        };
        match remove_index {
            Some(i) => {self.suitCounts.remove(i);()},
            None => (),
        };
        let r:&suit::Suit = &suitToRet;
        return Some(r);
    }
}

pub trait ValuePicker {
    fn value(&mut self) -> Option<&value::Value>;
}

struct RandomValuePicker;

impl ValuePicker for RandomValuePicker {
    fn value(&mut self) -> Option<&value::Value> {
        let valueIndex = rand::random::<uint>() % value::VALUES.len();
        return Some(&value::VALUES[valueIndex] as &value::Value);
    }
}

struct ValueCount {
    value: value::ValueImpl,
    counts: uint,
}

struct RandomDeckValuePicker {
    valueCounts: Vec<ValueCount>,
}

impl ValuePicker for RandomDeckValuePicker {
    fn value(&mut self) -> Option<&value::Value> {
        if self.valueCounts.len() == 0 {
            return None;
        }
        let (valueToRet, remove_index) = {
            let valueIndex = rand::random::<uint>() % self.valueCounts.len();
            let ref mut valueToLook = self.valueCounts.get_mut(valueIndex);
            let valueToRet = valueToLook.value;
            valueToLook.counts -= 1;
            if valueToLook.counts == 0 {
                (valueToRet, Some(valueIndex))
            } else {
                (valueToRet, None)
            }
        };
        match remove_index {
            Some(i) => {self.valueCounts.remove(i);()},
            None => (),
        }
        return Some(&valueToRet as &value::Value);
    }
}

pub struct GenericShoe<'a> {
    valuePicker: &'a mut ValuePicker + 'a,
    suitPickers: [&'a mut SuitPicker + 'a, ..13],
}
/*
impl shoe::DirectShoe for DirectRandomShoe {
}

impl DirectRandomShoe {
    fn new(numDeck: uint) ->  DirectRandomShoe {
        let mut c = Vec::new();
        for &v in value::VALUES.iter() {
            let mut sc = Vec::new();
            for &s in suit::SUITS.iter() {
                let scount = SuitCount {
                    suit: s,
                    counts: numDeck,
                };
                sc.push(scount);
            }
            let vc = ValueCount{
                value: v,
                counts: numDeck * 4,
                suitCounts: sc,
            };
            c.push(vc);
        }
        return DirectRandomShoe{
            len: numDeck * 52,
            cardCounts: c,
        };
    }
}

#[test]
fn test_random() {
    let mut randDeck = DirectRandomShoe::new(1);
    shoe::test_single_deck(&mut randDeck);
}
*/
