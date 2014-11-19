extern crate num;
use rules::BJRules;
use hand::BJHand;
use std::collections::Bitv;
use self::num::bigint::BigUint;
use self::num::Zero;
use self::num::bigint::ToBigUint;

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
    pub fn new(max_value: uint, current_value: uint) -> HashRange {
        assert!(max_value >= current_value);
        HashRange {
            max_value: max_value,
            current_value: current_value,
        }
    }
}

fn create_hash(ranges: &[HashRange]) -> Vec<u8> {
    let mut val: BigUint = Zero::zero();
    let mut bits_required: BigUint = Zero::zero();
    for &i in ranges.iter() {
        val = val * i.max_value.to_biguint().unwrap();
        val = val + i.current_value.to_biguint().unwrap();
        bits_required = bits_required * i.max_value.to_biguint().unwrap();
    }
    let mut bv = Bitv::with_capacity(bits_required.bits(), false);
    let mut current_bit = 0u;
    while val > 0u.to_biguint().unwrap() {
        bv.set(current_bit, val % 2u.to_biguint().unwrap() == 1u.to_biguint().unwrap());
        current_bit += 1;
        val = val / 2u.to_biguint().unwrap();
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

#[cfg(test)]
mod tests {
    extern crate test;
    use hand_hasher::create_hash;
    use hand_hasher::HashRange;
    #[test]
    fn test_create_hash() {
        let v : Vec<u8> = vec![3];
        let m : &[HashRange] = [HashRange::new(2, 1)];
        assert_eq!(v, create_hash(m));
    }
}
