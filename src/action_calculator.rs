use hand::BJHand;
use bjaction::BJAction;
use bjaction::BJAction::HIT;
use bjaction::BJAction::STAND;
use bjaction::BJAction::DOUBLE;
use bjaction::BJAction::SURRENDER;
use bjaction::BJAction::SPLIT;
use cards::card::Card;
use cards::value::VALUES;
use cards::value::ACE;
use cards::value::Value;
use cards::value::KING;
use cards::value::QUEEN;
use cards::value::JACK;
use cards::value::TEN;
use cards::value::NINE;
use rules::BJRules;
use shoe::shoe::DirectShoe;
use shoe::randomshoe::SuitPicker;
use hand_hasher::HandHasher;
use hand_hasher::DeckHasher;
use hash_database::HashDatabase;

pub struct ActionCalculator<'a> {
    player_hand_hasher: &'a (HandHasher + 'a),
    deck_hasher: &'a (DeckHasher + 'a),
    database: &'a mut (HashDatabase + 'a),
}

impl <'a>ActionCalculator<'a> {
    fn dbget(&self, hash1: &Vec<u8>, hash2: &Vec<u8>) -> Option<f64> {
        let mut v3 = Vec::new();
        v3.push_all(hash1.as_slice());
        v3.push_all(hash2.as_slice());
        self.database.get(v3)
    }

    fn dbstore(&mut self, hash1: &Vec<u8>, hash2: &Vec<u8>, value: f64) -> Option<f64> {
        let mut v3 = Vec::new();
        v3.push_all(hash1.as_slice());
        v3.push_all(hash2.as_slice());
        self.database.store(v3, value)
    }
    pub fn expected_value_best_action(&mut self, hand: &mut BJHand,
                                  dealer_up_card: &Card, d: &mut DirectShoe,
                                  rules: &BJRules) -> f64 {
        let v1 = self.player_hand_hasher.hash_hand(rules, hand);
        let v2 = self.deck_hasher.hash_deck(rules, d);
        match self.dbget(&v1, &v2) {
            Some(s) => return s,
            None => {}
        }
//        println!("Checking expected value for hand={}", hand);
        let mut best_result: Option<f64>  = None;
        // TODO: Can I just do something like BJAction.variants ??
        let actions = [STAND, HIT, DOUBLE, SPLIT, SURRENDER];
        let mut best_action = None;
        for a in actions.iter() {
            // TODO: This is temp.  Remove later
            if hand.score() < 17 && dealer_up_card.value() == &NINE && rules.can_hit(hand) && *a == STAND {
                    continue
            }
            if hand.score() > 16 && !hand.is_soft() && dealer_up_card.value() == &NINE && rules.can_stand(hand) && *a == HIT {
                    continue
            }
            match self.expected_value(hand, dealer_up_card, d, *a, rules) {
                Some(r) => {
                    match best_result {
                        Some(b) => {
                            if b < r {
                                best_result = Some(r);
                                best_action = Some(a);
                            }
                        }
                        None => {
                            best_result = Some(r);
                            best_action = Some(a);
                        }
                    }
                }
                _ => {}
            }
        }
        assert_eq!(v1, self.player_hand_hasher.hash_hand(rules, hand));
        assert!(best_result != None);
        let to_return = best_result.unwrap();
        println!("Best action {} => {}: {}", hand.simple_desc(), best_action.unwrap(), to_return);
        match self.dbstore(&v1, &v2, to_return) {
            Some(_) => {
                panic!("Logic loop????...")
            }
            None => {
                to_return
            }
        }
    }

    fn odds_of_value(&mut self, dealer_up_card: &Card,
                      d: &mut DirectShoe,
                      rules: &BJRules, v: &Value) -> f64 {
        use hand::score_for_value;
        let count_of_val = d.count(v);
        if count_of_val == 0 {
            return 0.0;
        }
        // Tricky: If the dealer is showing a TEN, then
        //         you're more likely to get a ACE than
        //         any other card because you know the
        //         dealer doesn't have one of your aces
        let (num_valid_down_cards, invalid_down_val) = {
            match d.initial_length() {
                Some(_) => {
                    if rules.dealer_blackjack_after_hand() {
                        (d.len(), None)
                    } else {
                        match score_for_value(dealer_up_card.value()) {
                            10 => (d.len() - d.count(&ACE), Some(score_for_value(&ACE))),
                            11 => (d.len() - d.count(&TEN) - d.count(&JACK) - d.count(&QUEEN) -
                               d.count(&KING), Some(score_for_value(&TEN))),
                            _ => (d.len(), None)
                        }
                    }
                },
                // The second None here is KIND OF a lie ... oh well
                None => (d.len(), None)
            }
        };
        // Deck is [1,2, 2, 3, 3, 4, 4, 4]
        //  Odds of a 3 (when you know the dealer doesn't have 4) are: 
        //   (2/5) * (1/7) + (3/5) * (2/7)
        // Odds of a 3 are odds of sum of:
        //  (1) Dealer has three * odds of another three
        //  (2) Dealer does not have three * odds of your three
        match d.initial_length() {
            None => count_of_val as f64 / d.len() as f64,
            Some(_) =>
                match invalid_down_val == Some(score_for_value(v)) {
                true =>
                  count_of_val as f64 / (d.len() - 1) as f64,
                false =>
                   (count_of_val as f64 / num_valid_down_cards as f64) *
                    ((count_of_val - 1) as f64 / (d.len() - 1) as f64)
                    +
                    ((num_valid_down_cards - count_of_val) as f64 /
                     num_valid_down_cards as f64) *
                    ((count_of_val) as f64 / (d.len() - 1) as f64),
            }
        }
    }

    pub fn expected_value(&mut self, hand: &mut BJHand, dealer_up_card: &Card,
                      d: &mut DirectShoe, action: BJAction,
                      rules: &BJRules) -> Option<f64> {
        use std::fmt;
        if !rules.can_take_action(hand, action) {
            return None;
        }
        return match action {
            HIT => {
                assert!(rules.can_hit(hand));
                let mut final_result = 0.0;
                let mut debug = String::new();
                for v in VALUES.iter() {
                    let odds_of_value = self.odds_of_value(dealer_up_card, d, rules, v);
                    if odds_of_value != 0.0 {
                        let card_from_deck = match d.remove(v) {
                            Some(c) => c,
                            None => {
                                panic!("Item should exist!");
                            }
                        };
//                        let prev = format!("{}", hand);
                        hand.add_card(&card_from_deck);
//                        println!("H{} => {}", prev, hand);
                        let ev_with_value =
                            self.expected_value_best_action(
                                hand,dealer_up_card, d, rules);
                        final_result += odds_of_value * ev_with_value;
                        debug = debug + format_args!(fmt::format, "{}*{} +", odds_of_value, ev_with_value).as_slice();
                        hand.remove_card(&card_from_deck);
                        d.insert(&card_from_deck);
                    }
                }
//                println!("H {} = {}", hand, debug);
                Some(final_result)
            }
            STAND => {
                assert!(rules.can_stand(hand));
                let this_hands_value = match rules.is_blackjack(hand) {
                    true => {
                        rules.blackjack_payout()
                    }
                    false => {
                        if hand.score() > 21 {
                            -1.0
                        } else {
                            let mut dealer_hand = BJHand::new();
                            dealer_hand.add_card(dealer_up_card);
//                            println!("checking against dealer hand {} w/ {}", hand, fmt(d));
                            self.expected_with_dealer(hand,
                                                      &mut dealer_hand,
                                                      d, rules)
                        }
                    }
                };
                //   The flow is backwards, but the math works: We resolve the
                // previous hand, and then try to resolve this hand.  The real
                // flow of the cards is people pick their own hand, THEN the
                // dealer deals her own cards.  This logic is more like:
                // We let the dealer resolve the first split, then pick cards
                // for the second then let the dealer resolve the second split.
                Some(this_hands_value + self.finish_splits(hand, dealer_up_card, d, rules))
            }
            DOUBLE => {
                assert!(rules.can_double(hand));
                let mut final_result = 0.0;
                // Note: We support DaS, but something like SaD wouldn't work
                //       with this flow.
                let mut current_hand = hand.without_split_information();
                for v in VALUES.iter() {
                    let odds_of_value = self.odds_of_value(dealer_up_card, d, rules, v);
                    if odds_of_value != 0.0 {
                        let card_from_deck = match d.remove(v) {
                            Some(c) => c,
                            None => {
                                panic!("Expect a value: logic error");
                            }
                        };
                        current_hand.add_card(&card_from_deck);
                        current_hand.add_double_count();
                        // Expected value is twice resolving this hand, PLUS resolving
                        // any left over splits...  it is *NOT* twice resolving this
                        // hand plus a card since we carry on the card inside the hand
                        let ev_with_value =
                            2.0 * self.expected_value_best_action(
                                &mut current_hand, dealer_up_card, d, rules);
                        final_result += odds_of_value * ev_with_value;
                        current_hand.remove_card(&card_from_deck);
                        current_hand.subtract_double_count();
                        d.insert(&card_from_deck);
                    }
                }
                Some(final_result + self.finish_splits(hand, dealer_up_card, d, rules))
            }
            SPLIT => {
                assert!(rules.can_split(hand));
                hand.split();
                // Split the hand, you'll get a hit here (since hit is
                // the only option).
                let final_result = self.expected_value_best_action(
                        hand, dealer_up_card, d, rules);
                hand.unsplit();
                Some(final_result)
            }
            SURRENDER => {
                assert!(rules.can_surrender(hand));
                // Note: Allows surrender after split
                Some(-0.5 + self.finish_splits(hand, dealer_up_card, d, rules))
            }
        }
    }

    fn finish_splits(&mut self, original_hand: &BJHand, dealer_up_card: &Card,
                      d: &mut DirectShoe, rules: &BJRules) -> f64 {
        match original_hand.splits_to_solve() > 0 {
            false => 0.0,
            true => {
//                println!("finishing {} w/ {}", original_hand, fmt(d));
                let mut index = 0u;
                for &c in original_hand.cards().iter() {
                    index += 1;
                    // We still consider the card that made the split
                    if index > 1 {
                        d.insert(&c);
                    }
                }
                let mut hand = original_hand.create_next_split_hand();
                let ret = self.expected_value_best_action(&mut hand, dealer_up_card, d, rules);
                index = 0;
                for &c in original_hand.cards().iter() {
                    index += 1;
                    if index > 1 {
                        d.remove(c.value()).unwrap();
                    }
                }
                ret
            }
        }
    }

    pub fn expected_with_dealer(&mut self, player_hand: &BJHand,
                            dealer_hand: &mut BJHand, d: &mut DirectShoe,
                            rules: &BJRules) -> f64 {
        if !rules.should_hit_dealer_hand(dealer_hand) {
            let dealer_score = dealer_hand.score();
            let player_score = player_hand.score();
            assert!(player_score <= 21);
            if dealer_score > 21 {
                return 1.0;
            } else  if dealer_score > player_score {
                return -1.0;
            } else if dealer_score < player_score {
                return 1.0;
            } else {
                return 0.0;
            }
        } else {
            // The dealer hits ... takes a random card
            let mut final_result = 0.0;
            // Limit the number of valid cards on the first hand if
            // you already nkow the dealer doesn't have blackjack
            let number_of_valid_cards = {
                if rules.dealer_blackjack_after_hand() ||
                    dealer_hand.len() != 1 {
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
                    let odds_of_value =
                        count_of_val as f64 /
                        number_of_valid_cards as f64;
                    let card_from_deck = match d.remove(&v) {
                        Some(c) => c,
                        None => {
                            panic!("Count positive, but couldn't remove!");
                        }
                    };
                    assert_eq!(card_from_deck.value().desc(), v.desc());
                    dealer_hand.add_card(&card_from_deck);
                    if rules.is_blackjack(dealer_hand) &&
                        !rules.dealer_blackjack_after_hand() {
                            // ignore
                    } else {
                        let ev_with_value = self.expected_with_dealer(
                            player_hand, dealer_hand, d, rules);
                        final_result += odds_of_value * ev_with_value;
                    }

                    dealer_hand.remove_card(&card_from_deck);
                    d.insert(&card_from_deck);
                }
            }
            return final_result;
        }
    }
}


#[cfg(test)]
mod tests {
    extern crate test;
    use action_calculator::ActionCalculator;
    use self::test::Bencher;
    use cards::value::Value;
    use shoe::shoe::DirectShoe;
    use hand::BJHand;
    use std::num::Float;
    use cards::value;
    use bjaction::BJAction;
    use bjaction::BJAction::STAND;
    use bjaction::BJAction::HIT;
    use rules::BJRules;
    use hand_hasher::PlayerHandHasher;
    use hand_hasher::SuitlessDeckHasher;
    use hash_database::InMemoryHashDatabase;

    fn check_value(dealer_cards: &Vec<Value>, player_cards: &Vec<Value>,
                   expected: f64) {
        use shoe::randomshoe::new_infinite_shoe;
        use rules::BJRules;
        let mut a = ActionCalculator {
            player_hand_hasher: &PlayerHandHasher,
            deck_hasher: &SuitlessDeckHasher,
            database: &mut InMemoryHashDatabase::new(),
        };
        let rules = BJRules::new();
        let mut shoe = new_infinite_shoe();
        let expansion = 1000000.0f64;
        assert_eq!(
            (expected * expansion) as int,
            (a.expected_with_dealer(
                &BJHand::new_from_deck(&mut shoe, player_cards).unwrap(),
                &mut BJHand::new_from_deck(&mut shoe, dealer_cards).unwrap(),
                &mut shoe,
                &rules) * expansion).round() as int);
    }

    fn check_best_value(dealer_up_card: &Value, player_cards: &Vec<Value>,
                        expected: f64) {
        let rules = BJRules::new();
        check_best_value_rules(dealer_up_card, player_cards, expected, &rules);
    }

    fn check_best_value_rules(dealer_up_card: &Value, player_cards: &Vec<Value>,
                              expected: f64, rules: &BJRules) {
        use shoe::randomshoe::new_infinite_shoe;
        check_best_value_rules_deck(dealer_up_card, player_cards, expected,
                                    rules,&mut new_infinite_shoe());
    }

    fn check_best_value_rules_deck(dealer_up_card: &Value, player_cards: &Vec<Value>,
                              expected: f64, rules: &BJRules, shoe: &mut DirectShoe) {
        let mut a = ActionCalculator {
            player_hand_hasher: &PlayerHandHasher,
            deck_hasher: &SuitlessDeckHasher,
            database: &mut InMemoryHashDatabase::new(),
        };
        let expansion = 1000000.0f64;
        assert_eq!(
            (expected * expansion) as int,
            (a.expected_value_best_action(
                &mut BJHand::new_from_deck(shoe, player_cards).unwrap(),
                &shoe.remove(dealer_up_card).unwrap(),
                shoe,
                rules) * expansion).round() as int);
    }

    fn check_best_value_rules_deck_action(dealer_up_card: &Value, player_cards: &Vec<Value>,
                              expected: Option<f64>, rules: &BJRules, shoe: &mut DirectShoe,
                              action: BJAction) {
        let mut a = ActionCalculator {
            player_hand_hasher: &PlayerHandHasher,
            deck_hasher: &SuitlessDeckHasher,
            database: &mut InMemoryHashDatabase::new(),
        };
        let expansion = 1000000.0f64;
        let v =  a.expected_value(
            &mut BJHand::new_from_deck(shoe, player_cards).unwrap(),
            &shoe.remove(dealer_up_card).unwrap(),
            shoe,
            action,
            rules);
        match v {
            Some(f) => assert_eq!(
                (expected.unwrap() * expansion) as int,
                (f * expansion).round() as int),
            None => assert_eq!(expected, None)
        }
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
    fn test_expected_best_value_surr_16_a() {
        let rules = BJRules::new_complex(true, 1, false, 1, false, false, false);
        check_best_value_rules(&value::ACE,   &vec![value::TEN, value::SIX], -0.5, &rules);
    }

    #[test]
    fn test_expected_best_value_11_6() {
        check_best_value(&value::SIX,   &vec![value::FIVE, value::SIX], 0.667380);
    }

    #[test]
    fn test_expected_best_value_88_10() {
        check_best_value(&value::TEN,   &vec![value::EIGHT, value::EIGHT], -0.480686);
    }

    #[test]
    fn test_expected_best_value_88_9() {
        check_best_value(&value::NINE,   &vec![value::EIGHT, value::EIGHT], -0.387228);
    }

    #[test]
    fn test_expected_best_value_11_a() {
        check_best_value(&value::ACE,   &vec![value::EIGHT, value::THREE], 0.143001);
    }

    #[test]
    fn test_expected_best_value_92_10() {
        check_best_value(&value::TEN,   &vec![value::NINE, value::TWO], 0.179689);
    }

    #[test]
    fn test_expected_best_value_aa_10() {
        check_best_value(&value::TEN,   &vec![value::ACE, value::ACE], 0.179689);
    }

    #[test]
    fn test_expected_best_value_aa_2() {
        check_best_value(&value::TWO,   &vec![value::ACE, value::ACE], 0.470641);
    }

    #[test]
    fn test_expected_best_value_44_6() {
        check_best_value(&value::SIX,   &vec![value::FOUR, value::FOUR], 0.151377);
    }


    #[test]
    fn test_expected_best_value_s18_9() {
        check_best_value(&value::NINE,   &vec![value::ACE, value::SEVEN], -0.100744);
    }

    #[test]
    fn test_expected_best_value_9_2() {
        // HIT
        check_best_value(&value::TWO,   &vec![value::FIVE, value::FOUR], 0.074446);
    }

    #[test]
    fn test_expected_best_value_9_3() {
        // DOUBLE
        check_best_value(&value::THREE,   &vec![value::FIVE, value::FOUR], 0.120816);
    }

    #[test]
    fn test_expected_best_value_18_a() {
        check_best_value(&value::ACE,    &vec![value::TEN, value::EIGHT]           , -0.100199);
    }

    #[test]
    fn test_expected_best_value_21_2() {
        check_value(&vec![value::TWO],   &vec![value::TEN, value::FIVE, value::SIX],  0.882007);
        check_value(&vec![value::TWO],   &vec![value::TEN, value::TEN]             ,  0.639987);
    }

    #[test]
    fn test_expected_best_value_15_9() {
        check_best_value(&value::NINE,  &vec![value::TEN, value::FIVE]             , -0.471578);
    }

    #[test]
    fn test_expected_best_value_16_7() {
        check_best_value(&value::SEVEN,  &vec![value::TEN, value::SIX]             , -0.414779);
    }

    #[test]
    fn test_expected_best_value_a6_6() {
        check_best_value(&value::SIX,  &vec![value::ACE, value::SIX]             , 0.256104);
    }

    #[test]
    fn test_expected_best_value_20_10() {
        check_value(&vec![value::TEN],   &vec![value::TEN, value::TEN]             ,  0.554538);
    }

    #[test]
    fn test_expected_best_value_1d_16_10() {
        use shoe::randomshoe::new_random_shoe;
        // http://wizardofodds.com/games/blackjack/appendix/9/1ds17r4/
        let rules = BJRules::new_complex(false, 3, false, 1, false, false, true);
        let mut shoe = new_random_shoe(1);
        check_best_value_rules_deck(
            &value::TEN,
            &vec![value::TEN, value::SIX],
            -0.5069292,
            &rules,
            &mut shoe);
    }

    #[test]
    fn test_expected_value_1d_16_10_stand() {
        use shoe::randomshoe::new_random_shoe;
        // http://wizardofodds.com/games/blackjack/appendix/9/1ds17r4/
        let rules = BJRules::new_complex(false, 3, false, 1, false, false, true);
        let mut shoe = new_random_shoe(1);
        check_best_value_rules_deck_action(
            &value::TEN,
            &vec![value::TEN, value::SIX],
            Some(-0.542952),
            &rules,
            &mut shoe,
            BJAction::STAND);
    }

    #[test]
    fn test_expected_value_1d_16_10_hit() {
        use shoe::randomshoe::new_random_shoe;
        // http://wizardofodds.com/games/blackjack/appendix/9/1ds17r4/
        let rules = BJRules::new_complex(false, 3, false, 1, false, false, true);
        let mut shoe = new_random_shoe(1);
        // Tricky: You know the dealer doesn't have an ace, so you're more likely
        //         than other random cards to get an ace
        check_best_value_rules_deck_action(
            &value::TEN,
            &vec![value::KING, value::SIX],
            Some(-0.5069291),
            &rules,
            &mut shoe,
            BJAction::HIT);
    }

    #[test]
    fn test_expected_value_1d_16_9_hit() {
        use shoe::randomshoe::new_random_shoe;
        // http://wizardofodds.com/games/blackjack/appendix/9/1ds17r4/
        let rules = BJRules::new_complex(false, 3, false, 1, false, false, true);
        let mut shoe = new_random_shoe(1);
        check_best_value_rules_deck_action(
            &value::NINE,
            &vec![value::KING, value::SIX],
            Some(-0.479306),
            &rules,
            &mut shoe,
            BJAction::HIT);
    }

    #[test]
    fn test_expected_value_split_10_vs_9() {
        use shoe::randomshoe::new_infinite_shoe;
        // http://wizardofodds.com/games/blackjack/appendix/9/1ds17r4/
        let rules = BJRules::new_complex(false, 3, false, 1, false, false, true);
        let mut shoe = new_infinite_shoe();
        check_best_value_rules_deck_action(
            &value::NINE,
            &vec![value::TEN, value::TEN],
            // Differs from the wizard: He assumes you keep splitting those tens
            Some(0.233059),
            &rules,
            &mut shoe,
            BJAction::SPLIT);
    }

    #[test]
    fn test_expected_value_split_99_vs_9() {
        use shoe::randomshoe::new_infinite_shoe;
        // http://wizardofodds.com/games/blackjack/appendix/9/1ds17r4/
        let rules = BJRules::new_complex(false, 3, false, 1, false, false, true);
        let mut shoe = new_infinite_shoe();
        check_best_value_rules_deck_action(
            &value::NINE,
            &vec![value::NINE, value::NINE],
            Some(-0.078010),
            &rules,
            &mut shoe,
            BJAction::SPLIT);
    }

    #[test]
    fn test_expected_value_hit_54_vs_9() {
        use shoe::randomshoe::new_infinite_shoe;
        // http://wizardofodds.com/games/blackjack/appendix/9/1ds17r4/
        let rules = BJRules::new_complex(false, 3, false, 1, false, false, true);
        let mut shoe = new_infinite_shoe();
        check_best_value_rules_deck_action(
            &value::NINE,
            &vec![value::FIVE, value::FOUR],
            Some(-0.052178),
            &rules,
            &mut shoe,
            BJAction::HIT);
    }

    #[test]
    fn test_expected_best_value_1d_10_9_vs_10() {
        use shoe::randomshoe::new_random_shoe;
        // http://wizardofodds.com/games/blackjack/appendix/9/1ds17r4/
        let rules = BJRules::new_complex(false, 3, false, 1, false, false, true);
        let mut shoe = new_random_shoe(1);
        check_best_value_rules_deck(
            &value::TEN,
            &vec![value::TEN, value::NINE],
            0.102517,
            &rules,
            &mut shoe);
    }

    #[test]
    fn test_expected_best_value_1d_10_3_vs_3() {
        use shoe::randomshoe::new_random_shoe;
        // http://wizardofodds.com/games/blackjack/appendix/9/1ds17r4/
        let rules = BJRules::new_complex(false, 3, false, 1, false, false, true);
        let mut shoe = new_random_shoe(1);
        check_best_value_rules_deck(
            &value::THREE,
            &vec![value::TEN, value::THREE],
            -0.265648,
            &rules,
            &mut shoe);
    }

    #[test]
    #[ignore]
    fn test_expected_best_value_1d_88_vs_10() {
        use shoe::randomshoe::new_faceless_random_shoe;
        // S17
        // http://wizardofodds.com/games/blackjack/appendix/9/1ds17r4/
        let rules = BJRules::new_complex(false, 3, false, 1, false, false, true);
        let mut shoe = new_faceless_random_shoe(1);
        check_best_value_rules_deck(
            &value::TEN,
            &vec![value::EIGHT, value::EIGHT],
            -0.446436,
            &rules,
            &mut shoe);
    }

    #[test]
    #[ignore]
    fn test_expected_best_value_1d_88_vs_9() {
        use shoe::randomshoe::new_faceless_random_shoe;
        // S17
        // http://wizardofodds.com/games/blackjack/appendix/9/1ds17r4/
        let rules = BJRules::new_complex(false, 3, false, 1, false, false, true);
        let mut shoe = new_faceless_random_shoe(1);
        check_best_value_rules_deck(
            &value::NINE,
            &vec![value::EIGHT, value::EIGHT],
            -0.401442,
            &rules,
            &mut shoe);
    }

    #[test]
    fn test_expected_best_value_1d_88_vs_9_sp1_nodas() {
        use shoe::randomshoe::new_faceless_random_shoe;
        // S17
        // Don's book(page 403)
        // Note, my code says not to hit 8,2,2,2,2 vs 9 but his book says HIT b/c it
        // assumes it's 16 vs 9.  That's why my expected value can be higher than his
        let rules = BJRules::new_complex(false, 1, false, 1, false, false, false);
        let mut shoe = new_faceless_random_shoe(1);
        check_best_value_rules_deck(
            &value::NINE,
            &vec![value::EIGHT, value::EIGHT],
            -0.429934,
            &rules,
            &mut shoe);
    }

    #[test]
    fn test_expected_best_value_1d_88_vs_9_sp1_das() {
        use shoe::randomshoe::new_faceless_random_shoe;
        // S17
        // Don's book(page 403)
        // Note, my code says not to hit 8,2,2,2,2 vs 9 but his book says HIT b/c it
        // assumes it's 16 vs 9.  That's why my expected value can be higher than his
        let rules = BJRules::new_complex(false, 1, false, 1, false, false, true);
        let mut shoe = new_faceless_random_shoe(1);
        check_best_value_rules_deck(
            &value::NINE,
            &vec![value::EIGHT, value::EIGHT],
            -0.406325,
            &rules,
            &mut shoe);
    }

    #[test]
    #[ignore]
    fn test_expected_best_value_1d_88_vs_9_sp2_nodas() {
        use shoe::randomshoe::new_faceless_random_shoe;
        // S17
        // Don's book(page 403)
        // Note, my code says not to hit 8,2,2,2,2 vs 9 but his book says HIT b/c it
        // assumes it's 16 vs 9.  That's why my expected value can be higher than his
        let rules = BJRules::new_complex(false, 2, false, 1, false, false, false);
        let mut shoe = new_faceless_random_shoe(1);
        check_best_value_rules_deck(
            &value::NINE,
            &vec![value::EIGHT, value::EIGHT],
            -0.427106,
            &rules,
            &mut shoe);
    }

    #[bench]
    fn bench_with_calc(b: &mut Bencher) {
        use cards::value;
        b.iter(|| {
            check_best_value(&value::SEVEN,  &vec![value::TEN, value::SIX]             , -0.414779);
        });
    }
}
