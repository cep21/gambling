use shoe::shoe::DirectShoe;
use cards::value::Value;
use cards::value::VALUES;
use cards::value::TEN;
use cards::value::JACK;
use cards::value::QUEEN;
use cards::value::KING;
use cards::card::Card;
use cards::suit::Suit;
use cards::suit::SUITS;
use std::rand;

pub trait SuitPicker {
    fn suit(&mut self) -> Option<Suit>;
    fn insert(&mut self, &Suit);
    fn remove(&mut self, &Suit) -> Option<Suit>;
    fn count(&self, s: &Suit) -> uint;
    fn len(&self) -> uint;
}

pub trait ValuePicker {
    fn value(&mut self) -> Option<Value>;
    fn count(&self, v: &Value) -> uint;
    fn remove(&mut self, v: &Value) -> Option<Value>;
    fn insert(&mut self, v: &Value);
    fn len(&self) -> uint;
}

pub struct CycleSuitPicker {
    suit_index: uint,
}

impl CycleSuitPicker {
    pub fn new() -> CycleSuitPicker {
        return CycleSuitPicker {
            suit_index: 0,
        }
    }
}

impl SuitPicker for CycleSuitPicker {
    fn suit(&mut self) -> Option<Suit> {
        self.suit_index += 1;
        return Some(SUITS[self.suit_index % 4]);
    }
    fn insert(&mut self, _: &Suit) {
        // Not needed
    }
    fn count(&self, _: &Suit) -> uint {
        return 1;
    }
    fn remove(&mut self, val: &Suit) -> Option<Suit>{
        return Some(SUITS[val.index()])
    }
    fn len(&self) -> uint {
        return 4;
    }
}

struct RandomSuitPicker;

impl SuitPicker for RandomSuitPicker {
    fn suit(&mut self) -> Option<Suit> {
        let suit_index = rand::random::<uint>() % SUITS.len();
        return Some(SUITS[suit_index % 4]);
    }
    fn insert(&mut self, _: &Suit) {
        // Infinite deck.  Does nothing
    }
    fn count(&self, _: &Suit) -> uint {
        return 1;
    }
    fn remove(&mut self, val: &Suit) -> Option<Suit>{
        return Some(SUITS[val.index()])
    }
    fn len(&self) -> uint {
        return 4;
    }
}


struct RandomValuePicker;

impl ValuePicker for RandomValuePicker {
    fn value(&mut self) -> Option<Value> {
        let value_index = rand::random::<uint>() % VALUES.len();
        Some(VALUES[value_index])
    }
    fn count(&self, _: &Value) -> uint {
        // Assumes full single deck
        return 4;
    }
    fn remove(&mut self, v: &Value) -> Option<Value> {
        return Some(VALUES[v.index()]);
    }
    fn insert(&mut self, _: &Value) {
        // infinite deck.  Nothing done
    }
    fn len(&self) -> uint {
        // Assume full single deck
        return 52;
    }
}

struct IntCount {
    value: uint,
    counts: uint,
}


pub struct RandomItemPicker {
    non_zero_index_counts: Vec<uint>,
    indexed_value_counts: Vec<IntCount>,
    size: uint,
}

impl RandomItemPicker {
    pub fn new(initial_count_each: uint, max_index: uint) -> RandomItemPicker {
        let mut non_zero_index_counts= Vec::new();
        let mut indexed_value_counts= Vec::new();
        for i in range(0, max_index) {
            non_zero_index_counts.push(i);
            indexed_value_counts.push(IntCount{value: i, counts: initial_count_each});
        }
        return RandomItemPicker {
            non_zero_index_counts: non_zero_index_counts,
            indexed_value_counts: indexed_value_counts,
            size: initial_count_each * max_index,
        };
    }

    pub fn get_index(&mut self, index_to_find: uint) -> Option<uint> {
        let mut current: uint = 0;
        // The first time value_index <= current, we take the last value
        let mut index_in_loop = 0;
        while index_in_loop < self.non_zero_index_counts.len() {
            let index = self.non_zero_index_counts[index_in_loop];
            if self.indexed_value_counts[index].counts > 0 {
                if current + self.indexed_value_counts[index].counts > index_to_find {
                    return Some(index);
                }
                current += self.indexed_value_counts[index].counts;
                index_in_loop += 1;
            } else {
                self.non_zero_index_counts.swap_remove(index_in_loop);
            }
        }
        return None;
    }

    fn value(&mut self) -> Option<uint> {
        if self.non_zero_index_counts.len() == 0 {
            return None;
        }
        if self.size == 0 {
            return None;
        }
        let value_to_ret = {
            let value_index = rand::random::<uint>() % self.size;
            let value_count_index_to_consider = match self.get_index(value_index) {
                Some(c) => c,
                None => {
                    panic!("Should never return none.  Logic error!");
                }
            };
            let ref mut value_to_look = self.indexed_value_counts.get_mut(
                value_count_index_to_consider).unwrap();
            let value_to_ret = value_to_look.value;
            if value_to_look.counts == 0 {
                None
            } else {
                value_to_look.counts -= 1;
                Some(value_to_ret)
            }
        };
        match value_to_ret {
            Some(val) => {
                self.size -= 1;
                return Some(val);
            },
            None => self.value()
        }
    }
    fn count(&self, v: uint) -> uint {
        return self.indexed_value_counts[v].counts;
    }
    fn remove(&mut self, val: uint) -> bool {
        let ref mut v = self.indexed_value_counts.get_mut(val).unwrap();
        match v.counts == 0 {
            true => false,
            false => {
                self.size -= 1;
                v.counts -= 1;
                true
            }
        }
    }
    fn insert(&mut self, val: uint) {
        if self.indexed_value_counts[val].counts == 0 {
            self.non_zero_index_counts.push(val);
        }
        self.indexed_value_counts[val].counts += 1;
        self.size += 1;
    }

    fn len(&self) -> uint {
        return self.size;
    }
}

pub struct RandomDeckValuePicker {
    item_picker: RandomItemPicker,
}


impl RandomDeckValuePicker {
    pub fn new(num_decks: uint) -> RandomDeckValuePicker {
        return RandomDeckValuePicker{
            item_picker: RandomItemPicker::new(4 * num_decks, VALUES.len()),
        };
    }
    pub fn new_faceless(num_decks: uint) -> RandomDeckValuePicker {
        let mut ip = RandomItemPicker::new(4 * num_decks, VALUES.len());
        ip.indexed_value_counts[TEN.index()].counts = 4 * 4 * num_decks;
        ip.indexed_value_counts[JACK.index()].counts = 0;
        ip.indexed_value_counts[QUEEN.index()].counts = 0;
        ip.indexed_value_counts[KING.index()].counts = 0;
        RandomDeckValuePicker{
            item_picker: ip,
        }
    }
}

impl ValuePicker for RandomDeckValuePicker {
    fn value(&mut self) -> Option<Value> {
        match self.item_picker.value() {
            Some(c) => Some(VALUES[c]),
            None => None
        }
    }
    fn count(&self, v: &Value) -> uint {
        self.item_picker.count(v.index())
    }
    fn remove(&mut self, val: &Value) -> Option<Value> {
        match self.item_picker.remove(val.index()) {
            true => Some(VALUES[val.index()]),
            false => None,
        }
    }
    fn insert(&mut self, val: &Value) {
        self.item_picker.insert(val.index())
    }
    fn len(&self) -> uint {
        return self.item_picker.len();
    }
}

pub struct RandomDeckSuitPicker {
    item_picker: RandomItemPicker,
}


impl RandomDeckSuitPicker {
    pub fn new(num_decks: uint) -> RandomDeckSuitPicker {
        return RandomDeckSuitPicker{
            item_picker: RandomItemPicker::new(num_decks, SUITS.len()),
        };
    }
}

impl SuitPicker for RandomDeckSuitPicker {
    fn suit(&mut self) -> Option<Suit> {
        match self.item_picker.value() {
            Some(c) => Some(SUITS[c]),
            None => None
        }
    }
    fn count(&self, v: &Suit) -> uint {
        self.item_picker.count(v.index())
    }
    fn remove(&mut self, val: &Suit) -> Option<Suit> {
        match self.item_picker.remove(val.index()) {
            true => Some(SUITS[val.index()]),
            false => None,
        }
    }
    fn insert(&mut self, val: &Suit) {
        self.item_picker.insert(val.index())
    }
    fn len(&self) -> uint {
        return self.item_picker.len();
    }
}


pub struct GenericDirectShoe<'a> {
    value_picker: Box<ValuePicker + 'a>,
    suit_pickers: Box<[Box<SuitPicker + 'a>]>,
    initial_length: Option<uint>,
    maximum_count_of_any_value: Option<uint>,
}
/*
impl <'a>GenericDirectShoe<'a> {
    pub fn new(value_picker: Box<ValuePicker>, suit_pickers: Box<[Box<SuitPicker>]>)
            -> GenericDirectShoe<'a> {
        return GenericDirectShoe {
            value_picker: value_picker,
            suit_pickers: suit_pickers,
        }
    }
}
*/
impl <'a>DirectShoe for GenericDirectShoe<'a> {
    fn pop(&mut self) -> Option<Card> {
        return match self.value_picker.value() {
            Some(v) => {
                let ref mut picker = self.suit_pickers[v.index()];
                match picker.suit() {
                    Some(s) => {
                        Some(Card::new(v, s))
                    },
                    None =>  {
                        panic!("Suit should never be empty for a value {}!", v.desc())
                    }
                }
            },
            None => {
                None
            },
        };
    }
    fn len(&self) -> uint {
        return self.value_picker.len();
    }
    fn count(&self, v: &Value) -> uint {
        return self.value_picker.count(v);
    }
    fn remove(&mut self, v: &Value) -> Option<Card> {
        return match self.value_picker.remove(v) {
            None => None,
            Some(val) => match self.suit_pickers[v.index()].suit() {
                Some(s) => {
                    Some(Card::new(val, s))
                },
                None => {
                    panic!("Suit should never be empty for a value {}!", v.desc())
                }
            }
        }
    }
    fn insert(&mut self, v: &Card) {
        self.value_picker.insert(v.value());
        self.suit_pickers[v.value().index()].insert(v.suit());
    }
    fn initial_length(&self) -> Option<uint> {
        self.initial_length
    }
    fn maximum_count_of_any_value(&self) -> Option<uint> {
        self.maximum_count_of_any_value
    }
}

pub fn new_random_shoe<'a>(num_decks: uint) -> GenericDirectShoe<'a> {
    let vp = RandomDeckValuePicker::new(num_decks);
    let mut sp : Vec<Box<SuitPicker>> = Vec::new();
    for _ in range (0, 13u) {
        sp.push(box RandomDeckSuitPicker::new(num_decks));
    }
    GenericDirectShoe {
        value_picker: box vp,
        suit_pickers: sp.into_boxed_slice(),
        initial_length: Some(num_decks * 52),
        maximum_count_of_any_value: Some(4 * num_decks),
    }
}

pub fn new_faceless_random_shoe<'a>(num_decks: uint) -> GenericDirectShoe<'a> {
    let vp = RandomDeckValuePicker::new_faceless(num_decks);
    let mut sp : Vec<Box<SuitPicker>> = Vec::new();
    for i in range (0, 13u) {
        match VALUES[i] {
            TEN =>  sp.push(box RandomDeckSuitPicker::new(num_decks * 4)),
            JACK =>  sp.push(box RandomDeckSuitPicker::new(0)),
            QUEEN =>  sp.push(box RandomDeckSuitPicker::new(0)),
            KING =>  sp.push(box RandomDeckSuitPicker::new(0)),
            _ => sp.push(box RandomDeckSuitPicker::new(num_decks)),
        }
    }
    GenericDirectShoe {
        value_picker: box vp,
        suit_pickers: sp.into_boxed_slice(),
        initial_length: Some(num_decks * 52),
        maximum_count_of_any_value: Some(4 * num_decks),
    }
}

pub fn new_infinite_shoe<'a>() -> GenericDirectShoe<'a> {
    let vp = RandomValuePicker;
    let mut sp : Vec<Box<SuitPicker>> = Vec::new();
    for _ in range (0, 13u) {
        sp.push(box RandomSuitPicker);
    }
    GenericDirectShoe {
        value_picker: box vp,
        suit_pickers: sp.into_boxed_slice(),
        initial_length: None,
        maximum_count_of_any_value: None,
    }
    //GenericDirectShoe::new(box vp, sp.into_boxed_slice())
}

#[test]
fn test_random() {
    use shoe::shoe::test_single_deck;
    let mut shoe = new_random_shoe(1);
    println!("starting");
    test_single_deck(&mut shoe);
}

#[test]
fn test_cycle_suit_picker() {
    use std::collections::HashSet;
    let mut s = CycleSuitPicker::new();
    let mut set = HashSet::new();
    for _ in range(0, 4u) {
        match s.suit() {
            Some(i) => {set.insert(i.index());()}
            None => panic!("Should not panic!"),
        }
    }
    assert_eq!(4, s.len());
}

