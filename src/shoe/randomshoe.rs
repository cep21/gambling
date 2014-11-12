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
    pub fn new(numDecks: uint) -> RandomDeckSuitPicker {
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
    fn remove(&mut self, v: &Value);
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
    fn remove(&mut self, v: &Value) {
        // infinite deck.  Nothing done
    }
}

struct ValueCount {
    value: ValueImpl,
    counts: uint,
}

pub struct RandomDeckValuePicker<'a> {
    nonZeroIndexCounts: Vec<uint>,
    indexedValueCounts: Vec<ValueCount>,
    size: uint,
}

impl <'a>RandomDeckValuePicker<'a> {
    pub fn new(numDecks: uint) -> RandomDeckValuePicker<'a> {
        let mut nonZeroIndexCounts = Vec::new();
        let mut indexedValueCounts = Vec::new();
        for &v in VALUES.iter() {
            let vc = box ValueCount{value: v, counts: numDecks * 4};
            nonZeroIndexCounts.push(indexedValueCounts.len());
            indexedValueCounts.push(vc);
        }
        return RandomDeckValuePicker{
            valueCounts: valueCounts,
            indexedValueCounts: indexedValueCounts,
            size: numDecks * VALUES.len(),
        };
    }
}

impl <'a>ValuePicker for RandomDeckValuePicker<'a> {
    fn value(&mut self) -> Option<ValueImpl> {
        if self.valueCounts.len() == 0 {
            return None;
        }
        let (valueToRet, remove_index) = {
            let valueIndex = rand::random::<uint>() % self.valueCounts.len();
            let ref mut valueToLook = self.valueCounts.get_mut(valueIndex);
            let valueToRet = valueToLook.value;
            println!("count is {} for {}", valueToLook.counts, valueToLook.value.desc());
            if valueToLook.counts == 0 {
                (None, Some(valueIndex))
            } else {
                valueToLook.counts -= 1;
                if valueToLook.counts == 0 {
                    (Some(valueToRet), Some(valueIndex))
                } else {
                    (Some(valueToRet), None)
                }
            }
        };
        match remove_index {
            Some(i) => {self.valueCounts.remove(i);()},
            None => (),
        }
        match valueToRet {
            Some(val) => {
                return Some(val);
            },
            None => self.value()
        }
    }
    fn count(&self, v: &Value) -> uint {
        return self.indexedValueCounts[v.index()].counts;
    }
    fn remove(&mut self, val: &Value) {
        let ref mut v = self.indexedValueCounts.get_mut(val.index());
        println!("Counts is {}", v.counts);
        if v.counts == 0 {
//            fail!("Counts is zero!");
        } else {
            v.counts -= 1;
        }
    }
}

pub struct GenericDirectShoe<'a> {
    valuePicker: Box<ValuePicker + 'a>,
    suitPickers: Box<[Box<SuitPicker + 'a>]>,
    len: uint,
}

impl <'a>GenericDirectShoe<'a> {
    pub fn new(valuePicker: Box<ValuePicker>, suitPickers: Box<[Box<SuitPicker>]>, len: uint)
            -> GenericDirectShoe<'a> {
        return GenericDirectShoe {
            valuePicker: valuePicker,
            suitPickers: suitPickers,
            len: len,
        }
    }
}

impl <'a>DirectShoe for GenericDirectShoe<'a> {
    fn pop(&mut self) -> Option<CardImpl> {
        return match self.valuePicker.value() {
            Some(v) => {
                let ref mut picker = self.suitPickers[v.index()];
                match picker.suit() {
                    Some(s) => {
                        self.len -= 1;
                        Some(CardImpl::new(v, s))
                    },
                    None =>  {
                        fail!("Suit should never be empty for a value {}!", v.desc())
                    }
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
    fn remove(&mut self, v: &ValueImpl) -> Option<CardImpl> {
        self.valuePicker.remove(v);
        let ref mut picker = self.suitPickers[v.index()];
        match picker.suit() {
            Some(s) => {
                self.len -= 1;
                println!("Removed something!");
                Some(CardImpl::new(*v, s))
            },
            None => {
                return None;
            }
        }
    }
    fn insert(&mut self, v: &Card) {
        unimplemented!()
    }
}

#[test]
fn test_random() {
    use shoe::shoe::test_single_deck;
    let mut vp = RandomDeckValuePicker::new(1);
    let mut sp : Vec<Box<SuitPicker>> = Vec::new();
    for i in range (0, 13u) {
        sp.push(box RandomDeckSuitPicker::new(1));
    }
    let mut shoe = GenericDirectShoe::new(box vp, sp.into_boxed_slice(), 52);
    println!("starting");
    test_single_deck(&mut shoe);
}
