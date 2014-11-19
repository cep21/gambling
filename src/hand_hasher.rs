use rules::BJRules;
use hand::BJHand;
pub trait HandHasher {
    fn hash_hand(rules: &BJRules, hand: &BJHand) -> Vec<u8>;
}

pub struct DealerHandHasher;

impl HandHasher for DealerHandHasher {
    fn hash_hand(rules: &BJRules, hand: &BJHand) -> Vec<u8> {
        // The dealer's hand is only important on 17 if the dealer hits S17
        let mut score = hand.score();
        // All scores > 22 are the same to us
        if score > 22 {
            score = 22;
        }
        // Treat soft 18 same as hard 18 if the dealer stands on both
        let is_soft = match hand.is_soft() {
            false => false,
            true => match rules.dealer_hits_soft_score(hand.score()) {
                false => false,
                true => true,
            }
        };
        // Hash together the score and softness
        assert!(score <= 21);
        let mut hash: u8 = score as u8;
        if is_soft {
            hash += 22;
        }
        vec![hash]
    }
}
