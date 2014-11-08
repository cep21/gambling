use hand::BJHand;
use hand::BJHandImpl;
use bjaction::BJAction;
use bjaction::HIT;
use bjaction::STAND;
use bjaction::DOUBLE;
use bjaction::SURRENDER;
use bjaction::SPLIT;
use cards::card::Card;
use rules::BJRules;
use shoe::shoe::DirectShoe;

pub trait ActionCalculator {
    fn expectedValue(&self, h: &BJHand, dealerUpCard: &Card, d: &DirectShoe, action: BJAction, rules: &BJRules) ->
        Option<f64>;
    fn expectedWithDealer(&self, playerHand: &BJHand, dealerHand: &BJHand, d: &DirectShoe, rules:
                          &BJRules) -> Option<f64>;
}

pub struct ActionCalculatorImpl;

impl ActionCalculator for ActionCalculatorImpl {
    fn expectedValue(&self, h: &BJHand, dealerUpCard: &Card, d: &DirectShoe, action: BJAction, rules: &BJRules) ->
        Option<f64> {
        return match action {
            HIT => Some(0.0),
            STAND => {
                Some(0.0)
                // Find a dealer result that 
            },
            DOUBLE => Some(0.0),
            SPLIT => Some(0.0),
            SURRENDER => Some(0.5),
        }
    }
}

#[test]
fn test_expected_21() {
//    let mut randDeck = DirectRandomShoe::new(1);
//    shoe::test_single_deck(&mut randDeck); 
}
