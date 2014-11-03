use hand::BJHand;
use bjaction::BJAction;
use rules::BJRules;
use shoe::shoe::DirectShoe;

pub trait ActionCalculator {
    fn expectedValue(h: BJHand, d: DirectShoe, action: BJAction, rules: BJRules) -> f64;
}

pub struct ActionCalculatorImpl;

impl ActionCalculator for ActionCalculatorImpl {
    fn expectedValue(&self, h: BJHand, d: DirectShoe, action: BJAction, rules: BJRules) -> f64 {
    }
}
