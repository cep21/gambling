use cards::value;
use cards::value::Value;
use cards::card::Card;
use std::fmt;
use shoe::shoe::DirectShoe;

pub const INDEX_TO_SCORE: [u32;13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10];

pub fn score_for_value(v: &Value) -> u32 {
    return INDEX_TO_SCORE[v.index()];
}

pub struct BJHand {
    score: u32,
    ace_count: u32,
    splits_done: u32,
    num_cards: u32,
    double_count: u32,
    // The first two cards are very important for blackjack
    cards: Vec<Card>,
    splits_to_solve: Vec<Card>,
}

impl fmt::Show for BJHand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}|p={}|d={}|s={:?}",
               self.cards, self.splits_done,
               self.double_count, self.splits_to_solve)
    }
}

impl BJHand {
    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn score(&self) -> u32 {
        if self.is_soft() {
            return self.score + 10;
        } else {
            return self.score;
        }
    }

    pub fn is_soft(&self) -> bool {
        return self.ace_count > 0 && self.score + 10 <= 21;
    }

    pub fn simple_desc(&self) -> String {
        let mut s = String::new();
        if self.is_soft() {
            s = s + "S";
        } else {
            s = s + "H";
        }
        s.push_str(self.score().to_string().as_slice());
        return s;
    }

    pub fn split(&mut self) {
        assert_eq!(2, self.cards.len());
        assert_eq!(self.cards[0].value(), self.cards[1].value());
        let card_to_remove = self.cards[1];
        self.splits_to_solve.push(card_to_remove);
        self.remove_card(&card_to_remove);
    }

    pub fn unsplit(&mut self) {
        assert!(self.splits_to_solve.len() > 0);
        // Force them to remove the previous cards and put them back in the shoe
        let c = self.splits_to_solve.pop().unwrap();
        assert_eq!(1, self.cards.len());
        assert_eq!(c.value(), self.cards[0].value());
        self.add_card(&c);
    }

    pub fn create_next_split_hand(&self) -> BJHand {
        assert!(self.splits_to_solve.len() > 0);
        let mut ret = BJHand::new();
        ret.splits_done = self.splits_done + 1;
        // TODO: This could be more efficient.  push_all not working right,
        //       and can add one at a time.  Also, can I use a ref here?
        for i in self.splits_to_solve.iter() {
            ret.splits_to_solve.push(*i);
        }
        let card_to_add = ret.splits_to_solve.pop().unwrap();
        ret.add_card(&card_to_add);
        ret
    }

    pub fn without_split_information(&self) -> BJHand {
        let mut ret = BJHand::new_with_cards(&self.cards);
        // For redoubles
        ret.double_count = self.double_count;
        ret
    }

    pub fn double_count(&self) -> u32 {
        self.double_count
    }

    pub fn add_double_count(&mut self) {
        self.double_count += 1
    }

    pub fn splits_done(&self) -> u32 {
        self.splits_done
    }

    pub fn subtract_double_count(&mut self) {
        assert!(self.double_count >= 1);
        self.double_count -= 1;
    }

    pub fn len(&self) -> u32 {
        assert_eq!(self.num_cards, self.cards.len() as u32);
        return self.num_cards;
    }

    pub fn split_number(&self) -> u32 {
        self.splits_done + self.splits_to_solve.len() as u32
    }

    pub fn splits_to_solve(&self) -> u32 {
        self.splits_to_solve.len() as u32
    }

    pub fn add_card(&mut self, card: &Card) -> &mut BJHand {
        self.score += score_for_value(card.value());
        self.num_cards += 1;
        if card.value().index() == value::ACE.index() {
            self.ace_count += 1;
        }
        self.cards.push(card.clone());
        self
    }

    pub fn remove_card(&mut self, card: &Card) {
        assert!(self.score >= score_for_value(card.value()));
        assert!(self.num_cards >= 1);
        self.score -= score_for_value(card.value());
        self.num_cards -= 1;
        if card.value().index() == value::ACE.index() {
            assert!(self.ace_count >= 1);
            self.ace_count -= 1;
        }
        // We assume it's most likely you'll remove a card at the end
        for i in range(0, self.cards.len()).rev() {
            let should_remove = {
                let ref c = self.cards[i];
                if c.suit().index() == card.suit().index() && c.value().index() == card.value().index() {
                    true
                } else {
                    false
                }
            };
            if should_remove {
                self.cards.swap_remove(i);
                return;
            }
        }
        panic!("Could not find the card in the hand");
    }

    pub fn new() -> BJHand {
        return BJHand{
            score: 0,
            ace_count: 0,
            splits_done: 0,
            splits_to_solve: Vec::with_capacity(5),
            num_cards: 0,
            double_count: 0,
            cards: Vec::with_capacity(16),
        }
    }

    pub fn new_with_cards(cards: &Vec<Card>) -> BJHand {
        let mut h = BJHand::new();
        for c in cards.iter() {
            h.add_card(c);
        }
        return h;
    }

    pub fn new_from_deck(deck: &mut DirectShoe, values: &Vec<Value>) -> Option<BJHand> {
        let mut h = BJHand::new();
        for v in values.iter() {
            match deck.remove(v) {
                Some(c) => {
                    h.add_card(&c);
                }
                None => return None
            }
        }
        return Some(h);
    }
}

#[test]
fn test_hand() {
    use cards::suit;
    let mut h = BJHand::new();
    assert_eq!(0, h.score());
    assert_eq!(0, h.len());
    assert_eq!(false, h.is_soft());
    h.add_card(&Card::new(value::TEN, suit::SPADE));
    assert_eq!(1, h.len());
    assert_eq!(10, h.score());
    assert_eq!(false, h.is_soft());
    h.add_card(&Card::new(value::ACE, suit::SPADE));
    assert_eq!(2, h.len());
    assert_eq!(21, h.score());
    assert_eq!(true, h.is_soft());
    h.add_card(&Card::new(value::TWO, suit::SPADE));
    assert_eq!(3, h.len());
    assert_eq!(13, h.score());
    assert_eq!(false, h.is_soft());
    assert_eq!(3, h.len());
    h.add_card(&Card::new(value::KING, suit::SPADE));
    assert_eq!(4, h.len());
    assert_eq!(23, h.score());
    assert_eq!(false, h.is_soft());
    assert_eq!(4, h.len());
}
