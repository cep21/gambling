use cards::value;
use cards::value::Value;
use cards::card::Card;
use std::fmt;
use shoe::shoe::DirectShoe;

pub const INDEX_TO_SCORE: [uint, ..13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10];

pub fn score_for_value(v: &Value) -> uint {
    return INDEX_TO_SCORE[v.index()];
}

pub struct BJHand {
    score: uint,
    ace_count: uint,
    splits_done: uint,
    splits_to_solve: uint,
    num_cards: uint,
    double_count: uint,
    // The first two cards are very important for blackjack
    cards: Vec<Card>,
}

impl fmt::Show for BJHand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cards)
    }
}


impl BJHand {
    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn score(&self) -> uint {
        if self.is_soft() {
            return self.score + 10;
        } else {
            return self.score;
        }
    }

    pub fn is_soft(&self) -> bool {
        return self.ace_count > 0 && self.score + 10 <= 21;
    }

    pub fn split(&self) -> BJHand {
        assert_eq!(2, self.cards.len());
        assert_eq!(self.cards[0], self.cards[1]);
        return BJHand::new_split_hand(
            self.cards[0],
            self.splits_done + 1,
            self.splits_to_solve + 1);
    }

    pub fn double_count(&self) -> uint {
        self.double_count
    }

    pub fn add_double_count(&mut self) {
        self.double_count += 1
    }

    pub fn subtract_double_count(&mut self) {
        assert!(self.double_count >= 1);
        self.double_count -= 1;
    }

    pub fn len(&self) -> uint {
        assert_eq!(self.num_cards, self.cards.len());
        return self.num_cards;
    }

    pub fn split_number(&self) -> uint {
        return self.splits_done + self.splits_to_solve;
    }

    pub fn add_card(&mut self, card: Card) -> &mut BJHand {
        self.score += score_for_value(card.value());
        self.num_cards += 1;
        if card.value().index() == value::ACE.index() {
            self.ace_count += 1;
        }
        self.cards.push(card);
        self
    }

    pub fn remove_card(&mut self, card: Card) {
        assert!(self.score >= score_for_value(card.value()));
        assert!(self.num_cards >= 1);
        self.score -= score_for_value(card.value());
        self.num_cards -= 1;
        if card.value().index() == value::ACE.index() {
            assert!(self.ace_count >= 1);
            self.ace_count -= 1;
        }
        for i in range(0, self.cards.len()) {
            let c = self.cards[i];
            if c.suit().index() == card.suit().index() && c.value().index() == card.value().index() {
                self.cards.swap_remove(i);
                return;
            }
        }
        panic!("Could not find the card in the hand");
    }

    pub fn new_split_hand(card: Card, splits_done: uint, splits_to_solve: uint) -> BJHand {
        let mut h = BJHand {
            score: 0,
            ace_count: 0,
            splits_done: splits_done,
            splits_to_solve: splits_to_solve,
            num_cards: 0,
            double_count: 0,
            cards: Vec::new(),
        };
        h.add_card(card);
        h
    }

    pub fn new() -> BJHand {
        return BJHand{
            score: 0,
            ace_count: 0,
            splits_done: 0,
            splits_to_solve: 0,
            num_cards: 0,
            double_count: 0,
            cards: Vec::new(),
        }
    }

    pub fn new_with_cards(cards: &Vec<Card>) -> BJHand {
        let mut h = BJHand::new();
        for &c in cards.iter() {
            h.add_card(c);
        }
        return h;
    }

    pub fn new_from_deck(deck: &mut DirectShoe, values: &Vec<Value>) -> Option<BJHand> {
        let mut h = BJHand::new();
        for &v in values.iter() {
            match deck.remove(&v) {
                Some(c) => {
                    h.add_card(c);
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
    h.add_card(Card::new(value::TEN, suit::SPADE));
    assert_eq!(1, h.len());
    assert_eq!(10, h.score());
    assert_eq!(false, h.is_soft());
    h.add_card(Card::new(value::ACE, suit::SPADE));
    assert_eq!(2, h.len());
    assert_eq!(21, h.score());
    assert_eq!(true, h.is_soft());
    h.add_card(Card::new(value::TWO, suit::SPADE));
    assert_eq!(3, h.len());
    assert_eq!(13, h.score());
    assert_eq!(false, h.is_soft());
    assert_eq!(3, h.len());
    h.add_card(Card::new(value::KING, suit::SPADE));
    assert_eq!(4, h.len());
    assert_eq!(23, h.score());
    assert_eq!(false, h.is_soft());
    assert_eq!(4, h.len());
}
