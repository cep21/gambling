use hand::BJHandImpl;
use hand::BJHand;

pub trait WantCardDecision<'a> {
    fn shouldDeal(&self, hand: &BJHand) -> bool;
}

pub struct DealerCardDecision {
    hitSoft17: bool,
}

impl <'a>WantCardDecision<'a> for DealerCardDecision {
    fn shouldDeal(&self, hand: &BJHand) -> bool {
        let s = hand.score();
        if s < 17 {
            return true;
        }
        if s == 17 && hand.isSoft() {
            return true;
        }
        return false;
    }
}

#[test]
fn test_hand() {
    use cards::value;
    use cards::suit;
    use cards::card::Card;
    let d = DealerCardDecision{
        hitSoft17: true,
    };
    let h = &mut BJHandImpl::new();
    assert_eq!(true, d.shouldDeal(h))
    h.addCard(Card::new(value::TEN, suit::SPADE));
    assert_eq!(true, d.shouldDeal(h))
    h.addCard(Card::new(value::TWO, suit::SPADE));
    assert_eq!(true, d.shouldDeal(h))
    h.addCard(Card::new(value::ACE, suit::SPADE));
    assert_eq!(true, d.shouldDeal(h))
    h.addCard(Card::new(value::KING, suit::SPADE));
    assert_eq!(false, d.shouldDeal(h))
}
