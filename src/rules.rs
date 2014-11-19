use hand::BJHand;
use std::fmt;
use bjaction::BJAction;
use bjaction::BJAction::HIT;
use bjaction::BJAction::STAND;
use bjaction::BJAction::DOUBLE;
use bjaction::BJAction::SURRENDER;
use bjaction::BJAction::SPLIT;

pub struct BJRules{
    can_surrender: bool,
    split_limit: uint,
    hit_s17: bool,
    max_doubles_single_hand: uint,
}

impl BJRules {
    pub fn new() -> BJRules {
        BJRules::new_complex(false, 1, false, 1)
    }

    pub fn new_complex(can_surrender: bool, split_limit: uint, hit_s17: bool,
                       max_doubles_single_hand: uint) -> BJRules {
        BJRules {
            can_surrender: can_surrender,
            split_limit: split_limit,
            hit_s17: hit_s17,
            max_doubles_single_hand: max_doubles_single_hand,
        }
    }

    pub fn can_double(&self, h: &BJHand) -> bool {
        h.len() == 2 &&
            h.score() < 22 &&
            h.double_count() < self.max_doubles_single_hand
    }

    pub fn max_doubles_single_hand(&self) -> uint {
        self.max_doubles_single_hand
    }

    pub fn can_take_action(&self, h: &BJHand, action: BJAction) -> bool {
        match action {
            HIT => self.can_hit(h),
            STAND => self.can_stand(h),
            DOUBLE => self.can_double(h),
            SPLIT => self.can_split(h),
            SURRENDER => self.can_surrender(h),
        }
    }

    pub fn can_stand(&self, h: &BJHand) -> bool {
        h.len() > 1
    }

    pub fn can_split(&self, h: &BJHand) -> bool {
        h.split_number() < self.split_limit 
            && h.len() == 2
            && h.cards()[0].value() == h.cards()[1].value()
    }

    pub fn split_limit(&self) -> uint {
        self.split_limit
    }

    pub fn can_surrender(&self, h: &BJHand) -> bool {
        self.can_surrender && h.len() == 2 && h.split_number() == 0
    }

    pub fn should_hit_dealer_hand(&self, h: &BJHand) -> bool {
        h.score() < 17 || (self.hit_s17 && h.score() == 17 && h.is_soft())
    }

    pub fn dealer_hits_soft_score(&self, score: uint) -> bool {
        score < 17 || (self.hit_s17 && score == 17)
    }

    pub fn blackjack_payout(&self) -> f64 {
        1.5
    }

    pub fn automatic_win_at_hand_length(&self) -> uint {
        0
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
        let rules = BJRules::new_complex(true, 1, false, 1);
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
