use cards::value;
use cards::value::Value;
use cards::card;
use cards::card::Card;
use shoe::shoe;
use shoe::deck::cards_in_deck;
use shoe::shoe::DirectShoe;

pub struct DirectActualShoe<'a> {
    cards: &'a mut Vec<Card<'a>>,
}

impl <'a>shoe::DirectShoe<'a> for DirectActualShoe<'a> {
    fn pop(&mut self) -> Option<Card<'a>> {
        return self.cards.pop();
    }
    fn len(&self) -> uint {
        return self.cards.len();
    }
    fn count(&self, v: &value::Value) -> uint {
        let mut r = 0;
        for &c in self.cards.iter() {
            if c.value().index() == v.index() {
                r += 1;
            }
        }
        return r;
    }
    fn remove(&mut self, v: &Value) -> Option<Card<'a>> {
        return None;
    }
    fn insert(&mut self, v: &Card) {
    }
}

impl <'a> DirectActualShoe<'a> {
    /*
       TODO: Why does this not work?
    fn new(v: &'a mut Vec<card::Card>) -> DirectActualShoe<'a> {
        return DirectActualShoe{
            cards: v,
        }
    }*/
}

#[test]
fn test_direct() {
    let v = &mut Vec::new();
    let ds = DirectActualShoe::new(v);
    assert_eq!(0, ds.len());

    let v2 = &mut Vec::new();
    let mut ds2 = DirectActualShoe::new(cards_in_deck(1, v2));

//    shoe::test_single_deck(&mut ds2);
}
