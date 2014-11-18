use hand::BJHand;

pub trait BJRules {
    fn can_double(&self, h: &BJHand) -> bool;
    fn can_split(&self, h: &BJHand) -> bool;
    fn can_surrender(&self, h: &BJHand) -> bool;
    fn should_hit_dealer_hand(&self, h: &BJHand) -> bool;
    fn blackjack_payout(&self) -> f64;
    fn can_hit(&self, h: &BJHand) -> bool;
    fn dealer_blackjack_after_hand(&self) -> bool;
    fn is_blackjack(&self, &BJHand) -> bool;
}

pub struct BJRulesImpl;

impl BJRules for BJRulesImpl {
    fn can_double(&self, h: &BJHand) -> bool {
        return h.len() == 2;
    }
    fn can_split(&self, h: &BJHand) -> bool {
        h.split_number() < 4
    }
    fn can_surrender(&self, h: &BJHand) -> bool {
        h.len() == 2 && h.split_number() == 0
    }
    fn should_hit_dealer_hand(&self, h: &BJHand) -> bool {
        h.score() < 17
    }
    fn blackjack_payout(&self) -> f64 {
        1.5
    }
    fn can_hit(&self, h: &BJHand) -> bool {
        h.score() < 21
    }
    fn dealer_blackjack_after_hand(&self) -> bool {
        return false;
    }
    fn is_blackjack(&self, hand: &BJHand) -> bool {
        return hand.split_number() == 0
            && hand.len() == 2
            && hand.score() == 21;
    }
}
