use hand::BJHand;

pub trait BJRules {
    fn canDouble(&self, h: BJHand) -> bool;
    fn canSplit(&self, h: BJHand) -> bool;
    fn canSurrender(&self, h: BJHand) -> bool;
    fn shouldHitDealerHand(&self, h: &BJHand) -> bool;
    fn blackjackPayout(&self) -> f64;
    fn canHit(&self, h: BJHand) -> bool;
}

pub struct BJRulesImpl {
    hardDoubles: [bool, ..21],
}
