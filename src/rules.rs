use hand::BJHand;

pub trait BJRules {
    fn canDouble(&self, h: &BJHand) -> bool;
    fn canSplit(&self, h: &BJHand) -> bool;
    fn canSurrender(&self, h: &BJHand) -> bool;
    fn shouldHitDealerHand(&self, h: &BJHand) -> bool;
    fn blackjackPayout(&self) -> f64;
    fn canHit(&self, h: &BJHand) -> bool;
    fn dealer_blackjack_after_hand(&self) -> bool;
    fn is_blackjack(&self, &BJHand) -> bool;
}

pub struct BJRulesImpl;

impl BJRules for BJRulesImpl {
    fn canDouble(&self, h: &BJHand) -> bool {
        return h.len() == 2;
    }
    fn canSplit(&self, h: &BJHand) -> bool {
        // TODO:
        return false;
    }
    fn canSurrender(&self, h: &BJHand) -> bool {
        h.len() == 2 && h.splitNumber() == 0
    }
    fn shouldHitDealerHand(&self, h: &BJHand) -> bool {
        h.score() < 17
    }
    fn blackjackPayout(&self) -> f64 {
        1.5
    }
    fn canHit(&self, h: &BJHand) -> bool {
        h.score() < 21
    }
    fn dealer_blackjack_after_hand(&self) -> bool {
        return false;
    }
    fn is_blackjack(&self, hand: &BJHand) -> bool {
        return hand.splitNumber() == 0
            && hand.len() == 2
            && hand.score() == 21;
    }
}
