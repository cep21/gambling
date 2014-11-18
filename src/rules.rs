use hand::BJHand;

pub struct BJRules{
    can_surrender: bool,
}

impl BJRules {
    pub fn new() -> BJRules {
        BJRules {
            can_surrender: false,
        }
    }

    pub fn new_complex(can_surrender: bool) -> BJRules {
        BJRules {
            can_surrender: can_surrender,
        }
    }

    pub fn can_double(&self, h: &BJHand) -> bool {
        return h.len() == 2;
    }

    pub fn can_split(&self, h: &BJHand) -> bool {
        h.split_number() < 4
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
        h.score() < 21
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
