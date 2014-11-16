use hand::BJHand;
use bjaction::BJAction;
use bjaction::HIT;
use bjaction::STAND;
use bjaction::DOUBLE;
use bjaction::SURRENDER;
use bjaction::SPLIT;
use hand::BJHandImpl;
use cards::card::Card;
use cards::value::VALUES;
use cards::value::Value;
use rules::BJRules;
use shoe::shoe::DirectShoe;
use shoe::randomshoe::SuitPicker;

pub trait ActionCalculator {
    fn expected_value(&self, h: &mut BJHand, dealerUpCard: &Card,
                      d: &mut DirectShoe, action: BJAction,
                      rules: &BJRules) -> Option<f64>;
    fn expected_with_dealer(&self, player_hand: &BJHand, dealer_hand: &mut BJHand,
                            d: &mut DirectShoe, rules: &BJRules) -> Option<f64>;
}

pub struct ActionCalculatorImpl;

impl ActionCalculator for ActionCalculatorImpl {
    fn expected_value(&self, hand: &mut BJHand, dealerUpCard: &Card,
                      d: &mut DirectShoe, action: BJAction,
                      rules: &BJRules) -> Option<f64> {
        return match action {
            HIT => {
                match rules.canHit(hand) {
                    false => None,
                    true => Some(0.0),
                }
            }
            STAND => {
                let mut dealer_hand = BJHandImpl::new();
                dealer_hand.addCard(*dealerUpCard);
                self.expected_with_dealer(hand, &mut dealer_hand, d, rules)
                // Find a dealer result that 
            }
            DOUBLE => {
                match rules.canDouble(hand) {
                    false => None,
                    true => Some(0.0),
                }
            }
            SPLIT => {
                match rules.canSplit(hand) {
                    false => None,
                    true => Some(0.0),
                }
            }
            SURRENDER => {
                match rules.canSurrender(hand) {
                    false => None,
                    true => Some(0.5),
                }
            }
        }
    }
    fn expected_with_dealer(&self, player_hand: &BJHand, dealer_hand: &mut BJHand,
                            d: &mut DirectShoe, rules: &BJRules) -> Option<f64> {
        println!("Called {}", dealer_hand.score());
        if !rules.shouldHitDealerHand(dealer_hand) {
            println!("Early return");
            let dealer_score = dealer_hand.score();
            let player_score = player_hand.score();
            assert!(player_score <= 21);
            if dealer_score > 21 {
                return Some(1.0);
            } else  if dealer_score > player_score {
                return Some(-1.0);
            } else if dealer_score < player_score {
                return Some(1.0);
            } else {
                return Some(0.0);
            }
        } else {
            println!("Iterate");
            // The dealer hits ... takes a random card
            let mut final_result = 0.0;
            for &v in VALUES.iter() {
                let count_of_val = d.count(&v);
                if count_of_val > 0 {
                    let odds_of_value = count_of_val as f64 / d.len() as f64;
                    println!("Odds of {} is {}", v.desc(), odds_of_value);
                    let card_from_deck = match d.remove(&v) {
                        Some(c) => c,
                        None => {
                            fail!("Count positive, but couldn't remove!");
                        }
                    };
                    assert_eq!(card_from_deck.value().desc(), v.desc());
                    dealer_hand.addCard(card_from_deck);
                    if rules.is_blackjack(dealer_hand) &&
                        !rules.dealer_blackjack_after_hand() {
                            // ignore
                    } else {
                        let ev_with_value = match self.expected_with_dealer(
                            player_hand, dealer_hand, d, rules) {
                                Some(c) => c,
                                None => return None,
                        };
                        final_result += odds_of_value * ev_with_value;
                    }

                    dealer_hand.removeCard(card_from_deck);
                    d.insert(&card_from_deck);
                }
            }
            return Some(final_result);
        }
    }
}

fn get<T>(i: Option<T>) -> T {
    match i {
        Some(x) => x,
        None => fail!("Expect value")
    }
}

#[test]
fn test_expected_21() {
    use shoe::randomshoe::new_random_shoe;
    use cards::value;
    use rules::BJRulesImpl;
    let a = ActionCalculatorImpl;
    let rules = BJRulesImpl;
    let mut player_hand = BJHandImpl::new();
    let mut shoe = new_random_shoe(1);
    player_hand.addCard(get(shoe.remove(&value::TEN)));
    player_hand.addCard(get(shoe.remove(&value::TEN)));

    let mut dealer_hand = BJHandImpl::new();
    dealer_hand.addCard(get(shoe.remove(&value::TEN)));

    println!("starting");
    let result = a.expected_with_dealer(&player_hand,
                                        &mut dealer_hand,
                                        &mut shoe,
                                        &rules);
    println!("result is {}",result);
    match result {
        None => fail!("Should not happen"),
        Some(c) => {
            assert_eq!(535, (c * 1000.0) as int);
        }
    }
}
