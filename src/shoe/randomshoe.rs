use shoe::shoe::DirectShoe;
use cards::value::Value;
use cards::value::VALUES;
use cards::value::ValueImpl;
use cards::card::CardImpl;
use cards::suit::SuitImpl;
use cards::suit::Suit;
use cards::suit::SUITS;
use cards::card::Card;
use std::rand;

pub trait SuitPicker {
    fn suit(&mut self) -> Option<SuitImpl>;
    fn insert(&mut self, &Suit);
}

struct CycleSuitPicker {
    suit_index: uint,
}

impl SuitPicker for CycleSuitPicker {
    fn suit(&mut self) -> Option<SuitImpl> {
        self.suit_index += 1;
        return Some(SUITS[self.suit_index % 4]);
    }
    fn insert(&mut self, s: &Suit) {
        unimplemented!()
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
    fn insert(&mut self, s: &Suit) {
    }
}

pub trait ValuePicker {
    fn value(&mut self) -> Option<ValueImpl>;
    fn count(&self, v: &Value) -> uint;
    fn remove(&mut self, v: &Value);
    fn insert(&mut self, v: &Value);
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
    fn insert(&mut self, v: &Value) {
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
            let vc = ValueCount{value: v, counts: numDecks * 4};
            nonZeroIndexCounts.push(indexedValueCounts.len() as uint);
            indexedValueCounts.push(vc);
        }
        return RandomDeckValuePicker{
            nonZeroIndexCounts: nonZeroIndexCounts,
            indexedValueCounts: indexedValueCounts,
            size: numDecks * VALUES.len() * 4,
        };
    }

    pub fn get_index(&mut self, index_to_find: uint) -> Option<uint> {
        let mut current: uint = 0;
        // The first time valueIndex <= current, we take the last value
        let mut index_in_loop = 0;
        while index_in_loop < self.nonZeroIndexCounts.len() {
            let index = self.nonZeroIndexCounts[index_in_loop];
            if self.indexedValueCounts[index].counts > 0 {
                if current + self.indexedValueCounts[index].counts > index_to_find {
                    return Some(index);
                }
                current += self.indexedValueCounts[index].counts;
                index_in_loop += 1;
            } else {
                self.nonZeroIndexCounts.remove(index_in_loop);
            }
        }
        return None;
    }
}

impl <'a>ValuePicker for RandomDeckValuePicker<'a> {
    fn value(&mut self) -> Option<ValueImpl> {
        if self.nonZeroIndexCounts.len() == 0 {
            return None;
        }
        if self.size == 0 {
            return None;
        }
        let (valueToRet, remove_index) = {
            let valueIndex = rand::random::<uint>() % self.size;
            println!("Checking {} when size is {}", valueIndex, self.size);
            let valueCountIndexToConsider = match self.get_index(valueIndex) {
                Some(c) => c,
                None => {
                    fail!("Should never return none.  Logic error!");
                }
            };
            let ref mut valueToLook = self.indexedValueCounts.get_mut(
                valueCountIndexToConsider);
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
        match valueToRet {
            Some(val) => {
                self.size -= 1;
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
            self.size -= 1;
            v.counts -= 1;
        }
    }
    fn insert(&mut self, val: &Value) {
        if self.indexedValueCounts[val.index()].counts == 0 {
            self.nonZeroIndexCounts.push(val.index());
        }
        self.indexedValueCounts[val.index()].counts += 1;
        self.size += 1;
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
        self.valuePicker.insert(v.value());
        self.suitPickers[v.value().index()].insert(v.suit());
        self.len += 1;
    }
}

#[test]
fn test_random() {
    use shoe::shoe::test_single_deck;
    let vp = RandomDeckValuePicker::new(1);
    let mut sp : Vec<Box<SuitPicker>> = Vec::new();
    for i in range (0, 13u) {
        sp.push(box RandomDeckSuitPicker::new(1));
    }
    let mut shoe = GenericDirectShoe::new(box vp, sp.into_boxed_slice(), 52);
    println!("starting");
    test_single_deck(&mut shoe);
}
