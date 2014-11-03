use cards;
use cards::value;
use cards::suit;
use cards::value::Value;
use cards::card::CardImpl;
use cards::card::Card;

pub const INDEX_TO_SCORE: [uint, ..13] = [11, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10];
pub fn scoreForValue(v: &Value) -> uint {
    return INDEX_TO_SCORE[v.index()];
}

pub trait BJHand<'a> {
    fn score(&self) -> uint;
    fn isSoft(&self) -> bool;
    fn len(&self) -> uint;
    fn cards(&'a self) -> &'a Vec<CardImpl>;
    fn addCard(&mut self, CardImpl);
    fn splitNumber(&self) -> uint;
}

pub struct BJHandImpl<'a> {
    score: uint,
    cards: Vec<CardImpl>,
    hasAce: bool,
    splitNumber: uint,
}

impl <'a>BJHand<'a> for BJHandImpl<'a> {
    fn score(&self) -> uint {
        if self.hasAce {
            if self.score > 21 {
                return self.score - 10;
            } else {
                return self.score;
            }
        } else {
            return self.score;
        }
    }
    fn isSoft(&self) -> bool {
        return self.hasAce && self.score < 22;
    }
    fn len(&self) -> uint {
        return self.cards.len();
    }
    fn cards(&'a self) -> &'a Vec<CardImpl> {
        return &self.cards;
    }
    fn splitNumber(&self) -> uint {
        return self.splitNumber;
    }
    fn addCard(&mut self, card: CardImpl) {
        self.cards.push(card);
        self.score += scoreForValue(card.value());
        if card.value().index() == value::ACE.index() {
            self.hasAce = true;
        }
    }
}

impl <'a>BJHandImpl<'a> {
    pub fn new() -> BJHandImpl<'a> {
        return BJHandImpl{
            score: 0,
            cards: Vec::with_capacity(22),
            hasAce: false,
            splitNumber: 0,
        }
    }
}

#[test]
fn test_hand() {
    let mut h = BJHandImpl::new();
    assert_eq!(0, h.score());
    assert_eq!(0, h.len());
    assert_eq!(false, h.isSoft());
    h.addCard(CardImpl::new(value::TEN, suit::SPADE));
    assert_eq!(1, h.len());
    assert_eq!(10, h.score());
    assert_eq!(false, h.isSoft());
    h.addCard(CardImpl::new(value::ACE, suit::SPADE));
    assert_eq!(2, h.len());
    assert_eq!(21, h.score());
    assert_eq!(true, h.isSoft());
    h.addCard(CardImpl::new(value::TWO, suit::SPADE));
    assert_eq!(3, h.len());
    assert_eq!(13, h.score());
    assert_eq!(false, h.isSoft());
    assert_eq!(3, h.cards.len());
    h.addCard(CardImpl::new(value::KING, suit::SPADE));
    assert_eq!(4, h.len());
    assert_eq!(23, h.score());
    assert_eq!(false, h.isSoft());
    assert_eq!(4, h.cards.len());
}
