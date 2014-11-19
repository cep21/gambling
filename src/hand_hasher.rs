use rules::BJRules;
use hand::BJHand;
pub trait HandHasher {
    fn hash_hand(rules: &BJRules, hand: &BJHand) -> Vec<u8>;
}

pub struct DealerHandHasher;

impl HandHasher for DealerHandHasher {
    fn hash_hand(rules: &BJRules, hand: &BJHand) -> Vec<u8> {
        let mut score = hand.score();
        // All scores > 22 are the same to us
        if score > 22 {
            score = 22;
        }
        let is_soft = hand.is_soft() &&
        // Treat soft 17 same as hard 17 if the dealer stands on both
            rules.dealer_hits_soft_score(hand.score());

        // Hash together the score and softness
        assert!(score <= 22);
        let mut hash: u8 = score as u8;
        if is_soft {
            hash += 23;
        }
        vec![hash]
    }
}

pub struct PlayerHandHasher;

impl HandHasher for PlayerHandHasher {
    fn hash_hand(rules: &BJRules, hand: &BJHand) -> Vec<u8> {
        assert!(hand.len() >= 1);
        let mut score = hand.score();
        // All scores > 22 are the same to us
        if score > 22 {
            score = 22;
        }
        let is_soft = hand.is_soft();

        // Number of cards in the hand only matter if it's 1, 2, or 3+
        let cards_in_hand_hash = {
            if hand.len() == 1 {
                0u
            } else if hand.len() == 2 {
                1u
            } else {
                2u
            }
        };

        // Hash together the score and softness
        assert!(score <= 22);
        let mut hash: u8 = score as u8;
        if is_soft {
            hash += 22;
        }
        vec![hash]
    }
}
