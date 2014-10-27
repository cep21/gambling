use shoe;
use cards;
use suit;
use value;
use shoe::DirectShoe;
use std::rand;

struct SuitCount {
    suit: suit::SuitImpl,
    counts: uint,
}

struct ValueCount {
    value: value::ValueImpl,
    counts: uint,
    suitCounts: Vec<SuitCount>,
}

pub struct DirectRandomShoe {
   cardCounts: Vec<ValueCount>,
   len: uint,
}

impl shoe::DirectShoe for DirectRandomShoe {
    fn pop(&mut self) -> Option<cards::CardImpl> {
        if self.len == 0 {
            return None;
        }
        // TODO: must be better way than % rand
        let cardIndex = rand::random::<uint>() % self.cardCounts.len();
        let (valueToRet, suitToRet, remove_value_index) = {
            let ref mut cardToLook = self.cardCounts.get_mut(cardIndex);
            let valueToRet = cardToLook.value;
            cardToLook.counts -= 1;

            let suitIndex = rand::random::<uint>() % cardToLook.suitCounts.len();
            let (suitToRet, remove_index) = {
                let ref mut suitToLook = cardToLook.suitCounts.get_mut(suitIndex);
                let suitToRet = suitToLook.suit;
                suitToLook.counts -= 1;
                if suitToLook.counts == 0 {
                    (suitToRet, true)
                } else {
                    (suitToRet, false)
                }
            };
            if remove_index {
                cardToLook.suitCounts.remove(suitIndex);
            }
            if cardToLook.counts == 0 {
                (valueToRet, suitToRet, true)
            } else {
                (valueToRet, suitToRet, false)
            }
        };
        if remove_value_index {
            self.cardCounts.remove(cardIndex);
        }
        self.len -= 1;
        return Some(cards::CardImpl{
            v: valueToRet,
            s: suitToRet,
        });
    }
    fn len(&self) -> uint {
        return self.len;
    }
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
