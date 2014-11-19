use hand::BJHand;
use std::fmt;

pub struct BJRules{
    can_surrender: bool,
    split_limit: uint,
}

impl BJRules {
    pub fn new() -> BJRules {
        BJRules::new_complex(false, 1)
    }

    pub fn new_complex(can_surrender: bool, split_limit: uint) -> BJRules {
        BJRules {
            can_surrender: can_surrender,
            split_limit: split_limit,
        }
    }

    pub fn can_double(&self, h: &BJHand) -> bool {
        h.len() == 2 && h.score() < 22
    }

    pub fn can_stand(&self, h: &BJHand) -> bool {
        h.len() > 1
    }

    pub fn can_split(&self, h: &BJHand) -> bool {
        h.split_number() < self.split_limit 
            && h.len() == 2
            && h.cards()[0].value() == h.cards()[1].value()
    }

    pub fn can_surrender(&self, h: &BJHand) -> bool {
        self.can_surrender && h.len() == 2 && h.split_number() == 0
    }

    pub fn should_hit_dealer_hand(&self, h: &BJHand) -> bool {
        h.score() < 17
    }

    pub fn blackjack_payout(&self) -> f64 {
        1.5
    }

    pub fn can_hit(&self, h: &BJHand) -> bool {
        h.score() < 21 && h.double_count() == 0
    }

    pub fn dealer_blackjack_after_hand(&self) -> bool {
        return false;
    }

    pub fn is_blackjack(&self, hand: &BJHand) -> bool {
        return hand.split_number() == 0
            && hand.len() == 2
            && hand.score() == 21;
    }
}

impl fmt::Show for BJRules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sur={};splits={}",
            self.can_surrender,
            self.split_limit)
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use rules::BJRules;
    use hand::BJHand;
    use cards::value;
    use cards::value::TEN;
    use shoe::randomshoe::new_infinite_shoe;

    #[test]
    fn test_rules_after_split() {
        let rules = BJRules::new_complex(true, 1);
        let mut shoe = new_infinite_shoe();
        let mut hand = BJHand::new_from_deck(
            &mut shoe, &vec![value::TEN, value::TEN]).unwrap();
        assert!(rules.can_hit(&hand));
        assert!(rules.can_double(&hand));
        assert!(rules.can_stand(&hand));
        assert!(rules.can_surrender(&hand));

        hand.split();
        assert!(rules.can_hit(&hand));
        assert!(!rules.can_double(&hand));
        assert!(!rules.can_surrender(&hand));
        assert!(!rules.can_stand(&hand));

        hand.unsplit();
        assert!(rules.can_hit(&hand));
        assert!(rules.can_double(&hand));
        assert!(rules.can_stand(&hand));
        assert!(rules.can_surrender(&hand));
    }
}
