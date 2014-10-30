use shoe;
use cards;
use cards::CardImpl;
use cards::Card;
use value;
use shoe::DirectShoe;

pub struct DirectActualShoe<'a> {
    cards: &'a mut Vec<cards::CardImpl>,
}

impl <'a>shoe::DirectShoe for DirectActualShoe<'a> {
    fn pop(&mut self) -> Option<cards::CardImpl> {
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
}

impl <'a> DirectActualShoe<'a> {
    fn new(v: &'a mut Vec<cards::CardImpl>) -> DirectActualShoe<'a> {
        return DirectActualShoe{
            cards: v,
        }
    }
}

#[test]
fn test_direct() {
    use deck;
    let v = &mut Vec::new();
    let ds = DirectActualShoe::new(v);
    assert_eq!(0, ds.len());

    let v2 = &mut Vec::new();
    let mut ds2 = DirectActualShoe::new(deck::cards_in_deck(1, v2));

    shoe::test_single_deck(&mut ds2);
}
