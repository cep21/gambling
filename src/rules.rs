use hand::BJHand;

pub trait BJRules {
    fn canDouble(&self, h: &BJHand) -> bool;
    fn canSplit(&self, h: &BJHand) -> bool;
    fn canSurrender(&self, h: &BJHand) -> bool;
    fn shouldHitDealerHand(&self, h: &BJHand) -> bool;
    fn blackjackPayout(&self) -> f64;
    fn canHit(&self, h: &BJHand) -> bool;
}

pub struct BJRulesImpl;

impl BJRules for BJRulesImpl {
    fn canDouble(&self, h: &BJHand) -> bool {
        true
    }
    fn canSplit(&self, h: &BJHand) -> bool {
        true
    }
    fn canSurrender(&self, h: &BJHand) -> bool {
        true
    }
    fn shouldHitDealerHand(&self, h: &BJHand) -> bool {
        true
    }
    fn blackjackPayout(&self) -> f64 {
        1.5
    }
    fn canHit(&self, h: &BJHand) -> bool {
        h.score() < 21
    }
}
