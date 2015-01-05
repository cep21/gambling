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
use rules::BJRules;
use shoe::shoe::DirectShoe;
use shoe::randomshoe::SuitPicker;
use hand_hasher::HandHasher;
use hand_hasher::DeckHasher;
use hash_database::HashDatabase;
use hand_hasher::PlayerHandHasher;
use hand_hasher::HandScoreHasher;
use hand_hasher::DealerHandHasher;
use hand_hasher::SuitlessDeckHasher;
use hash_database::InMemoryHashDatabase;
use hand::score_for_value;
use time::TimeIt;

pub struct ActionCalculator<'a, 'b> {
    player_hand_hasher: Box<HandHasher + 'a>,
    dealer_hand_hasher: Box<HandHasher + 'a>,
    hand_score_hasher: Box<HandHasher + 'a>,
    deck_hasher: Box<DeckHasher + 'a>,
    database: Box<HashDatabase + 'a>,
    shoe: &'b mut (DirectShoe + 'b),
    rules: BJRules,
}

impl <'a, 'b>ActionCalculator<'a, 'b> {
    pub fn new(rules: BJRules, shoe: &'b mut (DirectShoe + 'b)) -> ActionCalculator<'a, 'b> {
        ActionCalculator {
            player_hand_hasher: box PlayerHandHasher,
            dealer_hand_hasher: box DealerHandHasher,
            hand_score_hasher: box HandScoreHasher,
            deck_hasher: box SuitlessDeckHasher,
            database: box InMemoryHashDatabase::new(),
            rules: rules,
            shoe: shoe,
        }
    }

    fn dbget(&self, hash: &Vec<u8>) -> Option<f64> {
        TimeIt::new("dbget");
        self.database.get(hash)
    }

    fn dbstore(&mut self, hash: &Vec<u8>, value: f64) -> Option<f64> {
        TimeIt::new("dbstore");
        self.database.store(hash, value)
    }

    pub fn total_expected_best_value(&mut self) -> f64 {
        TimeIt::new("total_expected_best_value");
        let mut total_ev = 0.0f64;
        let start_len = self.shoe.len();
        for dealer_up_value in VALUES.iter() {
            println!("on {}", dealer_up_value);
            if self.shoe.count(dealer_up_value) == 0 {
                continue;
            }
            let odds_of_dealer_up_value = (self.shoe.count(dealer_up_value) as f64) / (self.shoe.len() as f64);
            let dealer_up_card = self.shoe.remove(dealer_up_value).unwrap();
            // The game will deal itself the player's first two cards
            let mut hand = BJHand::new();
            let ev = self.expected_value_best_action(&mut hand, &dealer_up_card, false);
            println!("Discovered {} at {} over {} db size odds of {}", dealer_up_value, ev,
                     self.database.len(), odds_of_dealer_up_value);
            total_ev += odds_of_dealer_up_value * ev;
            self.shoe.insert(&dealer_up_card);
            assert_eq!(start_len, self.shoe.len());
        }
        return total_ev;
    }

    pub fn odds_of_blackjack(&self) -> f64 {
        let ace_count = self.shoe.count(&ACE);
        if ace_count == 0 {
            return 0.0;
        }
        let ten_count = self.shoe.count(&TEN) + self.shoe.count(&JACK) + self.shoe.count(&QUEEN) + self.shoe.count(&KING);
        if ten_count == 0 {
            return 0.0;
        }

        // Odds of (Ace + 10) + (10 + Ace)

        return (ace_count as f64 / self.shoe.len() as f64) * (ten_count as f64 / (self.shoe.len() as f64 - 1.0)) +
            (ten_count as f64/ self.shoe.len() as f64) * (ace_count as f64 / (self.shoe.len() as f64 - 1.0));
    }

    pub fn initial_hand(&self, hand: &BJHand) -> bool {
        return
            hand.len() == 2 &&
            hand.split_number() == 0 &&
            hand.double_count() == 0;
    }

    pub fn expected_value_best_action(&mut self, hand: &mut BJHand,
                                      dealer_up_card: &Card, has_dealer_checked_bj: bool) -> f64 {
        TimeIt::new("expected_value_best_action");
        let v1 = {
            let mut v2 = self.player_hand_hasher.hash_hand(&self.rules, hand);
            v2.push_all(self.deck_hasher.hash_deck(&self.rules, &*self.shoe).as_slice());
            v2.push(dealer_up_card.value().index() as u8);
            v2.push(has_dealer_checked_bj as u8);
            v2
        };
        match self.dbget(&v1) {
            Some(s) => return s,
            None => {}
        }
        if self.initial_hand(hand) && !self.rules.dealer_blackjack_after_hand() && !has_dealer_checked_bj {
            let mut odds_of_dealer_bj_and_no_self_bj = 0.0;
            for v in VALUES.iter() {
                let card_count = self.shoe.count(v);
                if card_count == 0 {
                    continue;
                }
                let odds_of_this_value = card_count as f64 / self.shoe.len() as f64;
                let down_dealer_card  = self.shoe.remove(v).unwrap();
                let dealer_hand = BJHand::new_with_cards(&vec![*dealer_up_card, down_dealer_card]);
                if self.rules.is_blackjack(&dealer_hand) {
                    if !self.rules.is_blackjack(hand) {
                        odds_of_dealer_bj_and_no_self_bj += odds_of_this_value;
                    }
                }
                self.shoe.insert(&down_dealer_card);
            }
            return (1.0 - odds_of_dealer_bj_and_no_self_bj) * self.expected_value_best_action(hand, dealer_up_card, true) + odds_of_dealer_bj_and_no_self_bj * -1.0;
        }
        let mut best_result: Option<f64>  = None;
        // TODO: Can I just do something like BJAction.variants ??
        let actions = [STAND, HIT, DOUBLE, SPLIT, SURRENDER];
        for a in actions.iter() {
            match self.expected_value(hand, dealer_up_card, *a, has_dealer_checked_bj) {
                Some(r) => {
                    match best_result {
                        Some(b) => {
                            if b < r {
                                best_result = Some(r);
                            }
                        }
                        None => {
                            best_result = Some(r);
                        }
                    }
                }
                _ => {}
            }
        }
        assert!(best_result != None);
        let to_return = best_result.unwrap();
        match self.dbstore(&v1, to_return) {
            Some(_) => {
                panic!("Logic loop????...")
            }
            None => {
                to_return
            }
        }
    }

    fn odds_of_value(&mut self, dealer_up_card: &Card, v: &Value) -> f64 {
        TimeIt::new("odds_of_value");
        let count_of_val = self.shoe.count(v);
        if count_of_val == 0 {
            return 0.0;
        }
        // Tricky: If the dealer is showing a TEN, then
        //         you're more likely to get a ACE than
        //         any other card because you know the
        //         dealer doesn't have one of your aces
        let (num_valid_down_cards, invalid_down_val) = {
            match self.shoe.initial_length() {
                Some(_) => {
                    if self.rules.dealer_blackjack_after_hand() {
                        (self.shoe.len(), None)
                    } else {
                        match score_for_value(dealer_up_card.value()) {
                            10 => (self.shoe.len() - self.shoe.count(&ACE), Some(score_for_value(&ACE))),
                            11 => (self.shoe.len() - self.shoe.count(&TEN) - self.shoe.count(&JACK) - self.shoe.count(&QUEEN) -
                               self.shoe.count(&KING), Some(score_for_value(&TEN))),
                            _ => (self.shoe.len(), None)
                        }
                    }
                },
                // The second None here is KIND OF a lie ... oh well
                None => (self.shoe.len(), None)
            }
        };
        // Deck is [1,2, 2, 3, 3, 4, 4, 4]
        //  Odds of a 3 (when you know the dealer doesn't have 4) are: 
        //   (2/5) * (1/7) + (3/5) * (2/7)
        // Odds of a 3 are odds of sum of:
        //  (1) Dealer has three * odds of another three
        //  (2) Dealer does not have three * odds of your three
        match self.shoe.initial_length() {
            None => count_of_val as f64 / self.shoe.len() as f64,
            Some(_) =>
                match invalid_down_val == Some(score_for_value(v)) {
                true =>
                  count_of_val as f64 / (self.shoe.len() - 1) as f64,
                false =>
                   (count_of_val as f64 / num_valid_down_cards as f64) *
                    ((count_of_val - 1) as f64 / (self.shoe.len() - 1) as f64)
                    +
                    ((num_valid_down_cards - count_of_val) as f64 /
                     num_valid_down_cards as f64) *
                    ((count_of_val) as f64 / (self.shoe.len() - 1) as f64),
            }
        }
    }

    pub fn expected_value(&mut self, hand: &mut BJHand, dealer_up_card: &Card,
                          action: BJAction, has_dealer_checked_bj: bool) -> Option<f64> {
        TimeIt::new("expected_value");
        if !self.rules.can_take_action(hand, action) {
            return None;
        }
        return match action {
            HIT => {
                assert!(self.rules.can_hit(hand));
                let mut final_result = 0.0;
                for v in VALUES.iter() {
                    let odds_of_value = self.odds_of_value(dealer_up_card, v);
                    if odds_of_value != 0.0 {
                        let card_from_deck = match self.shoe.remove(v) {
                            Some(c) => c,
                            None => {
                                panic!("Item should exist!");
                            }
                        };
                        hand.add_card(&card_from_deck);
                        let ev_with_value =
                            self.expected_value_best_action(
                                hand,dealer_up_card, has_dealer_checked_bj);
                        final_result += odds_of_value * ev_with_value;
                        hand.remove_card(&card_from_deck);
                        self.shoe.insert(&card_from_deck);
                    }
                }
                Some(final_result)
            }
            STAND => {
                assert!(self.rules.can_stand(hand));
                let this_hands_value = match self.rules.is_blackjack(hand) {
                    true => {
                        // Note: Current logic assumes dealer has already checked
                        //       for blackjack
                        self.rules.blackjack_payout()
                    }
                    false => {
                        if hand.score() > 21 {
                            -1.0
                        } else {
                            let mut dealer_hand = BJHand::new();
                            dealer_hand.add_card(dealer_up_card);
//                            println!("checking against dealer hand {} w/ {}", hand, fmt(d));
                            self.expected_with_dealer(hand, &mut dealer_hand)
                        }
                    }
                };
                //   The flow is backwards, but the math works: We resolve the
                // previous hand, and then try to resolve this hanself.shoe.  The real
                // flow of the cards is people pick their own hand, THEN the
                // dealer deals her own cards.  This logic is more like:
                // We let the dealer resolve the first split, then pick cards
                // for the second then let the dealer resolve the second split.
                Some(this_hands_value + self.finish_splits(hand, dealer_up_card,
                                                           has_dealer_checked_bj))
            }
            DOUBLE => {
                assert!(self.rules.can_double(hand));
                let mut final_result = 0.0;
                // Note: We support DaS, but something like SaD wouldn't work
                //       with this flow.
                let mut current_hand = hand.without_split_information();
                for v in VALUES.iter() {
                    let odds_of_value = self.odds_of_value(dealer_up_card, v);
                    if odds_of_value != 0.0 {
                        let card_from_deck = match self.shoe.remove(v) {
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
                                &mut current_hand, dealer_up_card, has_dealer_checked_bj);
                        final_result += odds_of_value * ev_with_value;
                        current_hand.remove_card(&card_from_deck);
                        current_hand.subtract_double_count();
                        self.shoe.insert(&card_from_deck);
                    }
                }
                Some(final_result + self.finish_splits(hand, dealer_up_card, has_dealer_checked_bj))
            }
            SPLIT => {
                assert!(self.rules.can_split(hand));
                hand.split();
                // Split the hand, you'll get a hit here (since hit is
                // the only option).
                let final_result = self.expected_value_best_action(
                        hand, dealer_up_card, has_dealer_checked_bj);
                hand.unsplit();
                Some(final_result)
            }
            SURRENDER => {
                assert!(self.rules.can_surrender(hand));
                // Note: Allows surrender after split
                Some(-0.5 + self.finish_splits(hand, dealer_up_card, has_dealer_checked_bj))
            }
        }
    }

    fn finish_splits(&mut self, original_hand: &BJHand, dealer_up_card: &Card,
                     has_dealer_checked_bj: bool) -> f64 {
        TimeIt::new("finish_splits");
        match original_hand.splits_to_solve() > 0 {
            false => 0.0,
            true => {
                let mut index = 0u;
                for &c in original_hand.cards().iter() {
                    index += 1;
                    // We still consider the card that made the split
                    if index > 1 {
                        self.shoe.insert(&c);
                    }
                }
                let mut hand = original_hand.create_next_split_hand();
                let ret = self.expected_value_best_action(&mut hand, dealer_up_card,
                                                          has_dealer_checked_bj);
                index = 0;
                for &c in original_hand.cards().iter() {
                    index += 1;
                    if index > 1 {
                        self.shoe.remove(c.value()).unwrap();
                    }
                }
                ret
            }
        }
    }

    pub fn expected_with_dealer(&mut self, player_hand: &BJHand,
                            dealer_hand: &mut BJHand) -> f64 {
        TimeIt::new("expected_with_dealer");
        if !self.rules.should_hit_dealer_hand(dealer_hand) {
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
            let v1 = {
                let mut v2 = self.dealer_hand_hasher.hash_hand(&self.rules, dealer_hand);
                v2.push_all(self.deck_hasher.hash_deck(&self.rules, &*self.shoe).as_slice());
                v2.push_all(self.hand_score_hasher.hash_hand(&self.rules, player_hand).as_slice());
                v2
            };
            match self.dbget(&v1) {
                Some(s) => return s,
                None => {}
            }
            // The dealer hits ... takes a random card
            let mut final_result = 0.0;
            // Limit the number of valid cards on the first hand if
            // you already nkow the dealer doesn't have blackjack
            let number_of_valid_cards = {
                if self.rules.dealer_blackjack_after_hand() ||
                    dealer_hand.len() != 1 {
                        self.shoe.len()
                } else {
                    match dealer_hand.score() {
                        10 => self.shoe.len() - self.shoe.count(&ACE),
                        11 => self.shoe.len() - self.shoe.count(&TEN) - self.shoe.count(&JACK) - self.shoe.count(&QUEEN) -
                           self.shoe.count(&KING),
                        _ => self.shoe.len()
                    }
                }
            };
            for &v in VALUES.iter() {
                let count_of_val = self.shoe.count(&v);
                if count_of_val > 0 {
                    let odds_of_value =
                        count_of_val as f64 /
                        number_of_valid_cards as f64;
                    let card_from_deck = match self.shoe.remove(&v) {
                        Some(c) => c,
                        None => {
                            panic!("Count positive, but couldn't remove!");
                        }
                    };
                    assert_eq!(card_from_deck.value().desc(), v.desc());
                    dealer_hand.add_card(&card_from_deck);
                    if self.rules.is_blackjack(dealer_hand) &&
                        !self.rules.dealer_blackjack_after_hand() {
                            // ignore
                    } else {
                        let ev_with_value = self.expected_with_dealer(
                            player_hand, dealer_hand);
                        final_result += odds_of_value * ev_with_value;
                    }

                    dealer_hand.remove_card(&card_from_deck);
                    self.shoe.insert(&card_from_deck);
                }
            }
            match self.dbstore(&v1, final_result) {
                Some(_) => {
                    panic!("Logic loop????...")
                }
                None => {
                    final_result
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    extern crate test;
    use action_calculator::ActionCalculator;
    use self::test::Bencher;
    use time::TimeFileSave;
    use time::TimeIt;
    use cards::value::Value;
    use shoe::shoe::DirectShoe;
    use hand::BJHand;
    use std::num::Float;
    use cards::value;
    use bjaction::BJAction;
    use shoe::randomshoe::new_faceless_random_shoe;
    use bjaction::BJAction::STAND;
    use bjaction::BJAction::HIT;
    use rules::BJRules;
    use shoe::randomshoe::new_infinite_shoe;

    fn check_value(dealer_cards: &Vec<Value>, player_cards: &Vec<Value>,
                   expected: f64) {
        use shoe::randomshoe::new_infinite_shoe;
        use rules::BJRules;
        let rules = BJRules::new();
        let shoe = &mut new_infinite_shoe();
        let player_hand = BJHand::new_from_deck(shoe, player_cards).unwrap();
        let mut dealer_hand = BJHand::new_from_deck(shoe, dealer_cards).unwrap();
        let mut a = ActionCalculator::new(rules, shoe);
        let expansion = 1000000.0f64;
        assert_eq!(
            (expected * expansion) as int,
            (a.expected_with_dealer(&player_hand, &mut dealer_hand)
                * expansion).round() as int);
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
        let player_hand = &mut BJHand::new_from_deck(shoe, player_cards).unwrap();
        let dealer_up_card = &shoe.remove(dealer_up_card).unwrap();
        let mut a = ActionCalculator::new(*rules, shoe);
        let expansion = 1000000.0f64;
        assert_eq!(
            (expected * expansion) as int,
            (a.expected_value_best_action(player_hand, dealer_up_card,true)
             * expansion).round() as int);
    }

    fn check_best_value_rules_deck_action(dealer_up_card: &Value, player_cards: &Vec<Value>,
                              expected: Option<f64>, rules: &BJRules, shoe: &mut DirectShoe,
                              action: BJAction) {
        let player_hand = &mut BJHand::new_from_deck(shoe, player_cards).unwrap();
        let dealer_up_card = &shoe.remove(dealer_up_card).unwrap();
        let mut a = ActionCalculator::new(*rules, shoe);
        let expansion = 1000000.0f64;
        let v =  a.expected_value(
            player_hand,
            dealer_up_card,
            action, true);
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
        check_best_value(&value::ACE,   &vec![value::EIGHT, value::EIGHT], -0.372535);
        check_best_value(&value::THREE,   &vec![value::EIGHT, value::EIGHT], 0.146187);
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
    fn test_expected_best_value_a7_9() {
        check_best_value(&value::NINE,   &vec![value::ACE, value::SEVEN], -0.100744);
    }

    #[test]
    fn test_expected_best_value_aa_2() {
        {
            check_best_value(&value::TWO,   &vec![value::ACE, value::ACE], 0.470641);
        }
        TimeFileSave::new("test_result.txt");
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
        {
            let _ = TimeIt::new("time_1");
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
        TimeFileSave::new("name");
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
    #[ignore]
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
    #[ignore]
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

    #[test]
    #[ignore]
    fn test_expected_best_value_1d_sp1_nodas() {
        let rules = &BJRules::new_complex(false, 1, false, 1, false, false, false);
        let shoe = &mut new_faceless_random_shoe(1);
        let mut a = ActionCalculator::new(*rules, shoe);
        let ev = {
            a.total_expected_best_value()
        };
        {
            TimeFileSave::new("run_results.txt");
        }
        assert_eq!(0.017431, ev);
    }

    #[test]
    fn test_blackjack_odds() {
        let rules = &BJRules::new_complex(false, 3, false, 1, false, false, true);
        let shoe = &mut new_faceless_random_shoe(1);
        let a = ActionCalculator::new(*rules, shoe);
        let expected = 0.048265;
        let expansion = 1000000.0f64;
        assert_eq!(
            (expected * expansion) as int,
            (a.odds_of_blackjack() * expansion).round() as int);
    }

    #[test]
    #[ignore]
    fn test_expected_best_value_inf_sp4_s17_das() {
        let rules = &BJRules::new_complex(false, 3, false, 1, false, false, true);
        let shoe = &mut new_infinite_shoe();
        let mut a = ActionCalculator::new(*rules, shoe);
        let ev = {
            a.total_expected_best_value()
        };
        {
            TimeFileSave::new("run_results.txt");
        }
        // Still not exactly right :(
        assert_eq!(-0.00511734, ev);
    }

    #[bench]
    fn bench_with_calc(b: &mut Bencher) {
        use cards::value;
        b.iter(|| {
            check_best_value(&value::SEVEN,  &vec![value::TEN, value::SIX]             , -0.414779);
        });
    }
}
