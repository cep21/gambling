use cards::value;
use cards::value::Value;
use cards::card::Card;
use std::fmt;

pub const INDEX_TO_SCORE: [uint, ..13] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10];
pub fn scoreForValue(v: &Value) -> uint {
    return INDEX_TO_SCORE[v.index()];
}

pub trait BJHand<'a> {
    fn score(&self) -> uint;
    fn isSoft(&self) -> bool;
    fn len(&self) -> uint;
    fn addCard(&mut self, Card);
    fn removeCard(&mut self, Card);
    fn splitNumber(&self) -> uint;
    fn cards(&self) -> &Vec<Card>;
}

pub struct BJHandImpl<'a> {
    score: uint,
    aceCount: uint,
    splitNumber: uint,
    numCards: uint,
    cards: Vec<Card>,
}

impl <'a>fmt::Show for BJHandImpl<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cards)
    }
}


impl <'a>BJHand<'a> for BJHandImpl<'a> {
    fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    fn score(&self) -> uint {
        if self.isSoft() {
            return self.score + 10;
        } else {
            return self.score;
        }
    }
    fn isSoft(&self) -> bool {
        return self.aceCount > 0 && self.score + 10 <= 21;
    }
    fn len(&self) -> uint {
        assert_eq!(self.numCards, self.cards.len());
        return self.numCards;
    }
    fn splitNumber(&self) -> uint {
        return self.splitNumber;
    }
    fn addCard(&mut self, card: Card) {
        self.score += scoreForValue(card.value());
        self.numCards += 1;
        if card.value().index() == value::ACE.index() {
            self.aceCount += 1;
        }
        self.cards.push(card);
    }

    fn removeCard(&mut self, card: Card) {
        assert!(self.score >= scoreForValue(card.value()));
        assert!(self.numCards >= 1);
        self.score -= scoreForValue(card.value());
        self.numCards -= 1;
        if card.value().index() == value::ACE.index() {
            assert!(self.aceCount >= 1);
            self.aceCount -= 1;
        }
        for i in range(0, self.cards.len()) {
            let c = self.cards[i];
            if c.suit().index() == card.suit().index() && c.value().index() == card.value().index() {
                self.cards.swap_remove(i);
                return;
            }
        }
        fail!("Could not find the card in the hand");
    }
}

impl <'a>BJHandImpl<'a> {
    pub fn new() -> BJHandImpl<'a> {
        return BJHandImpl{
            score: 0,
            aceCount: 0,
            splitNumber: 0,
            numCards: 0,
            cards: Vec::new(),
        }
    }
}

#[test]
fn test_hand() {
    use cards::suit;
    let mut h = BJHandImpl::new();
    assert_eq!(0, h.score());
    assert_eq!(0, h.len());
    assert_eq!(false, h.isSoft());
    h.addCard(Card::new(value::TEN, suit::SPADE));
    assert_eq!(1, h.len());
    assert_eq!(10, h.score());
    assert_eq!(false, h.isSoft());
    h.addCard(Card::new(value::ACE, suit::SPADE));
    assert_eq!(2, h.len());
    assert_eq!(21, h.score());
    assert_eq!(true, h.isSoft());
    h.addCard(Card::new(value::TWO, suit::SPADE));
    assert_eq!(3, h.len());
    assert_eq!(13, h.score());
    assert_eq!(false, h.isSoft());
    assert_eq!(3, h.len());
    h.addCard(Card::new(value::KING, suit::SPADE));
    assert_eq!(4, h.len());
    assert_eq!(23, h.score());
    assert_eq!(false, h.isSoft());
    assert_eq!(4, h.len());
}
