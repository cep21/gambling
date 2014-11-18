use hand::BJHand;

pub trait WantCardDecision<'a> {
    fn should_deal(&self, hand: &BJHand) -> bool;
}

pub struct DealerCardDecision {
    hit_soft17: bool,
}

impl <'a>WantCardDecision<'a> for DealerCardDecision {
    fn should_deal(&self, hand: &BJHand) -> bool {
        let s = hand.score();
        if s < 17 {
            return true;
        }
        if s == 17 && hand.is_soft() {
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
        hit_soft17: true,
    };
    let h = &mut BJHand::new();
    assert_eq!(true, d.should_deal(h))
    h.add_card(Card::new(value::TEN, suit::SPADE));
    assert_eq!(true, d.should_deal(h))
    h.add_card(Card::new(value::TWO, suit::SPADE));
    assert_eq!(true, d.should_deal(h))
    h.add_card(Card::new(value::ACE, suit::SPADE));
    assert_eq!(true, d.should_deal(h))
    h.add_card(Card::new(value::KING, suit::SPADE));
    assert_eq!(false, d.should_deal(h))
}
