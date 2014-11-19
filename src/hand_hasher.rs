use rules::BJRules;
use hand::BJHand;
use std::collections::Bitv;

pub trait HandHasher {
    fn hash_hand(rules: &BJRules, hand: &BJHand) -> Vec<u8>;
}

pub struct DealerHandHasher;

struct HashRange {
    // Should be inclusive (so 1 means 2 values (0 or 1))
    max_value: uint,
    // Should be <- max_value
    current_value: uint,
}

impl HashRange {
    fn new(max_value: uint, current_value: uint) -> HashRange {
        HashRange {
            max_value: max_value,
            current_value: current_value,
        }
    }
}

fn create_hash(ranges: &[HashRange]) -> Vec<u8> {
    let mut total_bits_required = 0u;
    for &i in ranges.iter() {
        let mut m = i.max_value;
        while m > 0 {
            total_bits_required += 1;
            m /= 2;
        }
    }
    let mut bv = Bitv::with_capacity(total_bits_required, false);
    let mut current_index = 0u;
    for &i in ranges.iter() {
        let mut m = i.max_value;
        let mut v = i.current_value;
        assert!(v <= m);
        while m > 0 {
            if v % 2 == 0 {
                bv.set(current_index, false);
            } else {
                bv.set(current_index, true);
            }
            current_index += 1;
            m /= 2;
            v /= 2;
        }
    }
    return bv.to_bytes();
}

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

        // TODO: Include rules.automatic_win_at_hand_length()
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
