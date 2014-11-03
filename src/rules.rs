use hand::BJHand;

pub trait BJRules {
    fn canDouble(h: BJHand) -> bool;
    fn canSplit(h: BJHand) -> bool;
    fn canSurrender(h: BJHand) -> bool;
    fn shouldHitDealerHand(h: BJHand) -> bool;
    fn blackjackPayout() -> f64;
    fn canHit(h: BJHand) -> bool;
}

pub struct BJRulesImpl {
    hardDoubles: [bool, ..21],
}
