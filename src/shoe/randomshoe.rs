use shoe::shoe::DirectShoe;
use cards::value::Value;
use cards::value::VALUES;
use cards::value::ValueImpl;
use cards::card::CardImpl;
use cards::suit::SuitImpl;
use cards::suit::SPADE;
use cards::suit::CLUB;
use cards::suit::HEART;
use cards::suit::DIAMOND;
use cards::suit::SUITS;
use cards::card::Card;
use std::rand;

pub trait SuitPicker {
    fn suit(&mut self) -> Option<SuitImpl>;
}

struct CycleSuitPicker {
    suitIndex: uint,
}

impl SuitPicker for CycleSuitPicker {
    fn suit(&mut self) -> Option<SuitImpl> {
        self.suitIndex += 1;
        return Some(SUITS[self.suitIndex % 4]);
    }
}

struct SuitCount {
    suit: SuitImpl,
    counts: uint,
}

pub struct RandomDeckSuitPicker {
    suitCounts: Vec<SuitCount>,
}


impl RandomDeckSuitPicker {
    fn new(numDecks: uint) -> RandomDeckSuitPicker {
        let mut v = Vec::new();
        for &s in SUITS.iter() {
            v.push(SuitCount{suit: s, counts: numDecks});
        }
        return RandomDeckSuitPicker{
            suitCounts: v,
        };
    }
}

impl SuitPicker for RandomDeckSuitPicker {
    fn suit(&mut self) -> Option<SuitImpl> {
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
        return Some(suitToRet);
    }
}

pub trait ValuePicker {
    fn value(&mut self) -> Option<ValueImpl>;
    fn count(&self, v: &Value) -> uint;
}

struct RandomValuePicker;

impl ValuePicker for RandomValuePicker {
    fn value(&mut self) -> Option<ValueImpl> {
        let valueIndex = rand::random::<uint>() % VALUES.len();
        return Some(VALUES[valueIndex]);
    }
    fn count(&self, v: &Value) -> uint {
        // Assumes full single deck
        return 4;
    }
}

struct ValueCount {
    value: ValueImpl,
    counts: uint,
}

pub struct RandomDeckValuePicker {
    valueCounts: Vec<ValueCount>,
    indexedValueCounts: Vec<ValueCount>,
}

impl RandomDeckValuePicker {
    fn new(numDecks: uint) -> RandomDeckValuePicker {
        let mut valueCounts = Vec::new();
        let mut indexedValueCounts = Vec::new();
        for &v in VALUES.iter() {
            let vc = ValueCount{value: v, counts: numDecks * 4};
            valueCounts.push(vc);
            indexedValueCounts.push(vc);
        }
        return RandomDeckValuePicker{
            valueCounts: valueCounts,
            indexedValueCounts: indexedValueCounts,
        };
    }
}

impl ValuePicker for RandomDeckValuePicker {
    fn value(&mut self) -> Option<ValueImpl> {
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
        return Some(valueToRet);
    }
    fn count(&self, v: &Value) -> uint {
        return self.indexedValueCounts[v.index()].counts;
    }
}

pub struct GenericDirectShoe<'a, 'b:'a> {
    valuePicker: &'a mut ValuePicker + 'a,
    suitPickers: &'a mut [&'b mut SuitPicker + 'b],
    len: uint,
}

impl <'a, 'b: 'a>GenericDirectShoe<'a, 'b> {
    fn new(valuePicker: &'a mut ValuePicker, suitPickers: &'b mut [&'a mut SuitPicker], len: uint) -> GenericDirectShoe<'a, 'b> {
        return GenericDirectShoe {
            valuePicker: valuePicker,
            suitPickers: suitPickers,
            len: len,
        }
    }
}

impl <'a, 'b>DirectShoe for GenericDirectShoe<'a, 'b> {
    fn pop(&mut self) -> Option<CardImpl> {
        return match self.valuePicker.value() {
            Some(v) => {
                let ref mut picker = self.suitPickers[v.index()];
                match picker.suit() {
                    Some(s) => {
                        self.len -= 1;
                        Some(CardImpl::new(v, s))
                    },
                    None => None
                }
            },
            None => {
                None
            },
        };
    }
    fn len(&self) -> uint {
        return self.len;
    }
    fn count(&self, v: &Value) -> uint {
        return self.valuePicker.count(v);
    }
    fn remove(&mut self, v: &Value) -> Option<CardImpl> {
        unimplemented!()
        return None;
    }
    fn insert(&mut self, v: &Card) {
        unimplemented!()
    }
}

#[test]
fn test_random() {
//    let mut randDeck = DirectRandomShoe::new(1);
//    shoe::test_single_deck(&mut randDeck); 
}
