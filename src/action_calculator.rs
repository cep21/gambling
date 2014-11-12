use hand::BJHand;
use hand::BJHandImpl;
use bjaction::BJAction;
use bjaction::HIT;
use bjaction::STAND;
use bjaction::DOUBLE;
use bjaction::SURRENDER;
use bjaction::SPLIT;
use cards::card::Card;
use cards::card::CardImpl;
use cards::value::VALUES;
use cards::value;
use cards::suit;
use cards::value::Value;
use rules::BJRules;
use rules::BJRulesImpl;
use shoe::shoe::DirectShoe;
use shoe::randomshoe::RandomDeckSuitPicker;
use shoe::randomshoe::SuitPicker;
use shoe::randomshoe::RandomDeckValuePicker;
use shoe::randomshoe::GenericDirectShoe;

pub trait ActionCalculator {
    fn expectedValue(&self, h: &mut BJHand, dealerUpCard: &Card, d: &mut DirectShoe, action: BJAction, rules: &BJRules) ->
        Option<f64>;
    fn expectedWithDealer(&self, playerHand: &BJHand, dealerHand: &mut BJHand, d: &mut DirectShoe, rules:
                          &BJRules) -> Option<f64>;
}

pub struct ActionCalculatorImpl;

impl ActionCalculator for ActionCalculatorImpl {
    fn expectedValue(&self, h: &mut BJHand, dealerUpCard: &Card, d: &mut DirectShoe, action: BJAction, rules: &BJRules) ->
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
    fn expectedWithDealer(&self, playerHand: &BJHand, dealerHand: &mut BJHand, d: &mut DirectShoe,
                          rules: &BJRules) -> Option<f64> {
        if !rules.shouldHitDealerHand(dealerHand) {
            let dealerScore = dealerHand.score();
            let playerScore = playerHand.score();
            assert!(playerScore <= 21);
            if dealerScore > 21 {
                return Some(1.0);
            } else  if dealerScore > playerScore {
                return Some(-1.0);
            } else if dealerScore < playerScore {
                return Some(1.0);
            } else {
                return Some(0.0);
            }
        } else {
            // The dealer hits ... takes a random card
            let mut finalResult = 0.0;
            for &v in VALUES.iter() {
                let countOfVal = d.count(&v);
                if countOfVal > 0 {
                    let oddsOfValue = countOfVal as f64 / d.len() as f64;
                    let cardFromDeck = match d.remove(&v) {
                        Some(c) => c,
                        None => {
                            fail!("Count positive, but couldn't remove!");
                            return None;
                        }
                    };
                    assert_eq!(cardFromDeck.value().desc(), v.desc());
                    dealerHand.addCard(cardFromDeck);
                    let evWithValue = match self.expectedWithDealer(playerHand, dealerHand, d, rules) {
                        Some(c) => c,
                        None => return None,
                    };
                    finalResult += oddsOfValue * evWithValue;
                    dealerHand.removeCard(cardFromDeck);
                }
            }
            return Some(finalResult);
        }
    }
}

#[test]
fn test_expected_21() {
/*    let a = ActionCalculatorImpl;
    let rules = BJRulesImpl;
    let mut playerHand = BJHandImpl::new();
    playerHand.addCard(CardImpl::new(value::TEN, suit::SPADE));
    playerHand.addCard(CardImpl::new(value::TEN, suit::SPADE));

    let mut dealerHand = BJHandImpl::new();
    dealerHand.addCard(CardImpl::new(value::TEN, suit::SPADE));

    let mut vp = RandomDeckValuePicker::new(1);
    let mut sp : Vec<Box<SuitPicker>> = Vec::new();
    for i in range (0, 13u) {
        sp.push(box RandomDeckSuitPicker::new(1));
    }
    let mut shoe = GenericDirectShoe::new(box vp, sp.into_boxed_slice(), 52);
    println!("starting");
    let result = a.expectedWithDealer(
        &playerHand, &mut dealerHand, &mut shoe, &rules);
    println!("result is {}\n",result);
    assert_eq!(Some(0.0), result);*/
}
