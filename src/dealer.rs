use hand;
use hand::BJHand;

pub trait WantCardDecision<'a> {
    fn shouldDeal(&self, hand: &hand::BJHand) -> bool;
}

pub struct DealerCardDecision {
    hitSoft17: bool,
}

impl <'a>WantCardDecision<'a> for DealerCardDecision {
    fn shouldDeal(&self, hand: &hand::BJHand) -> bool {
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
    use value;
    use suit;
    use cards::CardImpl;
    let d = DealerCardDecision{
        hitSoft17: true,
    };
    let h = &mut hand::BJHandImpl::new();
    assert_eq!(true, d.shouldDeal(h))
    h.addCard(CardImpl::new(value::TEN, suit::SPADE));
    assert_eq!(true, d.shouldDeal(h))
    h.addCard(CardImpl::new(value::TWO, suit::SPADE));
    assert_eq!(true, d.shouldDeal(h))
    h.addCard(CardImpl::new(value::ACE, suit::SPADE));
    assert_eq!(true, d.shouldDeal(h))
    h.addCard(CardImpl::new(value::KING, suit::SPADE));
    assert_eq!(false, d.shouldDeal(h))
}
