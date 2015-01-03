use cards::value;
use cards::value::Value;
use cards::card::Card;
use shoe::shoe::DirectShoe;

pub struct DirectActualShoe<'a> {
    pub cards: &'a mut Vec<Card>,
    pub initial_length: Option<uint>,
    pub maximum_count_of_any_value: Option<uint>,
}

impl <'a>DirectShoe for DirectActualShoe<'a> {
    fn pop(&mut self) -> Option<Card> {
        return self.cards.pop();
    }
    fn len(&self) -> uint {
        return self.cards.len();
    }
    fn count(&self, v: &value::Value) -> uint {
        let mut r = 0;
        for c in self.cards.iter() {
            if c.value().index() == v.index() {
                r += 1;
            }
        }
        return r;
    }
    fn remove(&mut self, v: &Value) -> Option<Card> {
        for i in range(0, self.cards.len()) {
            if (*self.cards)[i].value().index() == v.index() {
                return self.cards.swap_remove(i);
            }
        }
        return None;
    }
    fn insert(&mut self, v: &Card) {
        self.cards.push((*v).clone());
    }
    fn initial_length(&self) -> Option<uint> {
        self.initial_length
    }
    fn maximum_count_of_any_value(&self) -> Option<uint> {
        self.maximum_count_of_any_value
    }
}

impl <'a> DirectActualShoe<'a> {
    pub fn new(v: &'a mut Vec<Card>) -> DirectActualShoe<'a> {
        return DirectActualShoe{
            initial_length: Some(v.len()),
            maximum_count_of_any_value: Some(0),
            cards: v,
        }
    }
}

#[test]
fn test_direct() {
    use shoe::directshoe::DirectActualShoe;
    use shoe::shoe::test_single_deck;
    use shoe::deck::cards_in_deck;
    let v = &mut Vec::new();
    let ds = DirectActualShoe {
        cards: v,
        initial_length: Some(0),
        maximum_count_of_any_value: Some(0),
    };
    assert_eq!(0, ds.len());

    let mut ds2 = DirectActualShoe {
        cards: &mut cards_in_deck(1),
        initial_length: Some(52),
        maximum_count_of_any_value: Some(4),
    };

    test_single_deck(&mut ds2);
}
