use hand::BJHand;
use bjaction::BJAction;
use bjaction::HIT;
use bjaction::STAND;
use bjaction::DOUBLE;
use bjaction::SURRENDER;
use bjaction::SPLIT;
use cards::card::Card;
use cards::value::VALUES;
use cards::value::ACE;
use cards::value::KING;
use cards::value::QUEEN;
use cards::value::JACK;
use cards::value::TEN;
use rules::BJRules;
use shoe::shoe::DirectShoe;
use shoe::randomshoe::SuitPicker;

pub trait ActionCalculator {
    fn expected_value(&self, h: &mut BJHand, dealer_up_card: &Card,
                      d: &mut DirectShoe, action: BJAction,
                      rules: &BJRules) -> Option<f64>;
    fn expected_with_dealer(&self, player_hand: &BJHand, dealer_hand: &mut BJHand,
                            d: &mut DirectShoe, rules: &BJRules) -> Option<f64>;
    fn expected_value_best_action(&self, hand: &mut BJHand, dealer_up_card: &Card,
                      d: &mut DirectShoe,
                      rules: &BJRules) -> f64;
}

pub struct ActionCalculatorImpl;

impl ActionCalculator for ActionCalculatorImpl {
    fn expected_value_best_action(&self, hand: &mut BJHand, dealer_up_card: &Card,
                      d: &mut DirectShoe,
                      rules: &BJRules) -> f64 {
        if hand.score() > 21 {
            // Bust... loose bet
            return -1.0;
        }
        let actions = vec![HIT];//, DOUBLE, SPLIT, SURRENDER];
        let mut best_result = match self.expected_value(
                hand, dealer_up_card, d, STAND, rules) {
            Some(s) => s,
            None => panic!("You should always be able to stand"),
        };
        for &a in actions.iter() {
            match self.expected_value(hand, dealer_up_card, d, a, rules) {
                Some(r) => {
                    if best_result < r {
                        best_result = r;
                    }
                }
                _ => {}
            }
        }
        return best_result;
    }
    fn expected_value(&self, hand: &mut BJHand, dealer_up_card: &Card,
                      d: &mut DirectShoe, action: BJAction,
                      rules: &BJRules) -> Option<f64> {
        return match action {
            HIT => {
                match rules.can_hit(hand) {
                    false => None,
                    true => {
                        let mut final_result = 0.0;
                        for &v in VALUES.iter() {
                            let count_of_val = d.count(&v);
                            if count_of_val > 0 {
                                let odds_of_value = count_of_val as f64 / d.len() as f64;
                                let card_from_deck = match d.remove(&v) {
                                    Some(c) => c,
                                    None => {
                                        panic!("Count positive, but couldn't remove!");
                                    }
                                };
                                hand.add_card(card_from_deck);
                                let ev_with_value = self.expected_value_best_action(
                                    hand, dealer_up_card, d, rules);
                                final_result += odds_of_value * ev_with_value;
                                hand.remove_card(card_from_deck);
                                d.insert(&card_from_deck);
                            }
                        }
                        Some(final_result)
                    }
                }
            }
            STAND => {
                let mut dealer_hand = BJHand::new();
                dealer_hand.add_card(*dealer_up_card);
                self.expected_with_dealer(hand, &mut dealer_hand, d, rules)
            }
            DOUBLE => {
                match rules.can_double(hand) {
                    false => None,
                    true => Some(-1.0),
                }
            }
            SPLIT => {
                match rules.can_split(hand) {
                    false => None,
                    true => Some(-1.0),
                }
            }
            SURRENDER => {
                match rules.can_surrender(hand) {
                    false => None,
                    true => Some(0.5),
                }
            }
        }
    }
    fn expected_with_dealer(&self, player_hand: &BJHand, dealer_hand: &mut BJHand,
                            d: &mut DirectShoe, rules: &BJRules) -> Option<f64> {
        if !rules.should_hit_dealer_hand(dealer_hand) {
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
            // The dealer hits ... takes a random card
            let mut final_result = 0.0;
            let number_of_valid_cards = {
                if rules.dealer_blackjack_after_hand() || dealer_hand.len() != 1 {
                    d.len()
                } else {
                    match dealer_hand.score() {
                        10 => d.len() - d.count(&ACE),
                        11 => d.len() - d.count(&TEN) - d.count(&JACK) - d.count(&QUEEN) -
                           d.count(&KING),
                        _ => d.len()
                    }
                }
            };
            for &v in VALUES.iter() {
                let count_of_val = d.count(&v);
                if count_of_val > 0 {
                    let odds_of_value = count_of_val as f64 / number_of_valid_cards as f64;
                    let card_from_deck = match d.remove(&v) {
                        Some(c) => c,
                        None => {
                            panic!("Count positive, but couldn't remove!");
                        }
                    };
                    assert_eq!(card_from_deck.value().desc(), v.desc());
                    dealer_hand.add_card(card_from_deck);
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

                    dealer_hand.remove_card(card_from_deck);
                    d.insert(&card_from_deck);
                }
            }
            return Some(final_result);
        }
    }
}


#[cfg(test)]
mod tests {
    extern crate test;
    use action_calculator::ActionCalculatorImpl;
    use action_calculator::ActionCalculator;
    use self::test::Bencher;
    use cards::value::Value;
    use shoe::shoe::DirectShoe;
    use hand::BJHand;
    use std::num::Float;
    use cards::value;

    fn check_value(dealer_cards: &Vec<Value>, player_cards: &Vec<Value>, expected: f64) {
        use shoe::randomshoe::new_infinite_shoe;
        use rules::BJRulesImpl;
        let a = ActionCalculatorImpl;
        let rules = BJRulesImpl;
        let mut shoe = new_infinite_shoe();
        let expansion = 1000000.0;
        assert_eq!(
            (a.expected_with_dealer(
                &BJHand::new_from_deck(&mut shoe, player_cards).unwrap(),
                &mut BJHand::new_from_deck(&mut shoe, dealer_cards).unwrap(),
                &mut shoe,
                &rules).unwrap() * expansion).round(),
            expected * expansion);
    }

    fn check_best_value(dealer_up_card: &Value, player_cards: &Vec<Value>, expected: f64) {
        use shoe::randomshoe::new_infinite_shoe;
        use rules::BJRulesImpl;
        let a = ActionCalculatorImpl;
        let rules = BJRulesImpl;
        let mut shoe = new_infinite_shoe();
        let expansion = 1000000.0;
        assert_eq!(
            (a.expected_value_best_action(
                &mut BJHand::new_from_deck(&mut shoe, player_cards).unwrap(),
                &shoe.remove(dealer_up_card).unwrap(),
                &mut shoe,
                &rules) * expansion).round(),
            expected * expansion);
    }

    #[test]
    fn test_expected_infinite_deck() {
        // Numbers checked against http://wizardofodds.com/games/blackjack/appendix/1/
        check_value(&vec![value::FOUR],  &vec![value::TEN, value::EIGHT]           ,  0.175854);
        check_value(&vec![value::SIX],   &vec![value::TEN, value::SIX]             , -0.153699);
        check_value(&vec![value::SEVEN], &vec![value::TEN, value::TEN]             ,  0.773227);
        check_value(&vec![value::SEVEN], &vec![value::TEN, value::SEVEN]           , -0.106809);
        check_value(&vec![value::NINE],  &vec![value::TEN, value::FIVE]            , -0.543150);
        check_value(&vec![value::TEN],   &vec![value::TEN, value::TEN]             ,  0.554538);
        check_value(&vec![value::TEN],   &vec![value::TEN, value::FIVE, value::SIX],  0.962624);
        check_value(&vec![value::TEN],   &vec![value::TEN, value::SEVEN]           , -0.419721);
        check_value(&vec![value::ACE],   &vec![value::TEN, value::SIX]             , -0.666951);
        check_value(&vec![value::ACE],   &vec![value::TEN, value::EIGHT]           , -0.100199);
    }

    #[test]
    fn test_expected_best_value_infinite1() {
        check_best_value(&value::ACE,    &vec![value::TEN, value::EIGHT]           , -0.100199);
        check_value(&vec![value::TEN],   &vec![value::TEN, value::TEN]             ,  0.554538);
    }

    #[test]
    fn test_expected_best_value_infinite2() {
        check_value(&vec![value::TWO],   &vec![value::TEN, value::FIVE, value::SIX],  0.882007);
        check_value(&vec![value::TWO],   &vec![value::TEN, value::TEN]             ,  0.639987);
    }

    #[test]
    fn test_expected_best_value_infinite3() {
        check_best_value(&value::NINE,  &vec![value::TEN, value::FIVE]             , -0.471578);
    }

    #[test]
    fn test_expected_best_value_infinite4() {
        check_best_value(&value::SEVEN,  &vec![value::TEN, value::SIX]             , -0.414779);
    }

    #[bench]
    fn bench_with_calc(b: &mut Bencher) {
        use cards::value;
        b.iter(|| {
            check_best_value(&value::SEVEN,  &vec![value::TEN, value::SIX]             , -0.414779);
        });
    }
}
