extern crate num;
use rules::BJRules;
use hand::BJHand;
use std::collections::Bitv;
use self::num::bigint::BigUint;
use self::num::Zero;
use self::num::bigint::ToBigUint;

pub trait HandHasher {
    fn hash_hand(&self, rules: &BJRules, hand: &BJHand) -> Vec<u8>;
}

pub struct DealerHandHasher;

struct HashRange {
    // Should be inclusive (so 1 means 2 values (0 or 1))
    max_value: uint,
    // Should be < max_value
    current_value: uint,
}

impl HashRange {
    pub fn new(max_value: uint, current_value: uint) -> HashRange {
        assert!(max_value > 1); // if the max_value == 1, then omit the dimension
        assert!(current_value < max_value);
        HashRange {
            max_value: max_value,
            current_value: current_value,
        }
    }
}

fn create_hash(ranges: &[HashRange]) -> Vec<u8> {
    // TODO: This could be a lot more efficient ...
    let mut val: BigUint = Zero::zero();
    let mut bits_required: BigUint = Zero::zero();
    for &i in ranges.iter() {
        val = val * i.max_value.to_biguint().unwrap();
        val = val + i.current_value.to_biguint().unwrap();
        bits_required = bits_required * i.max_value.to_biguint().unwrap() +
            (i.max_value - 1).to_biguint().unwrap();
    }
    let mut bv = Bitv::with_capacity(bits_required.bits(), false);
    println!("val={}, bits_required={}", val, bits_required);
    let mut current_bit = 0u;
    while val > 0u.to_biguint().unwrap() {
        bv.set(current_bit, val % 2u.to_biguint().unwrap() == 1u.to_biguint().unwrap());
        current_bit += 1;
        val = val / 2u.to_biguint().unwrap();
    }
    println!("bv={}", bv);
    return bv.to_bytes();
}

impl HandHasher for DealerHandHasher {
    fn hash_hand(&self, rules: &BJRules, hand: &BJHand) -> Vec<u8> {
        let mut score = hand.score();
        // All scores > 22 are the same to us
        if score > 22 {
            score = 22;
        }

        // Hash together the score and softness
        assert!(score <= 22);
        create_hash([
                    HashRange::new(23, score),
                    HashRange::new(2,
        // Treat soft 17 same as hard 17 if the dealer stands on both
                                   match hand.is_soft() &&
                                         rules.dealer_hits_soft_score(hand.score()) {
                                       true => 1,
                                       false => 0,
                                         })])
    }
}

pub struct PlayerHandHasher;

impl HandHasher for PlayerHandHasher {
    fn hash_hand(&self, rules: &BJRules, hand: &BJHand) -> Vec<u8> {
        assert!(hand.len() >= 1);
        let mut score = hand.score();
        // All scores > 22 are the same to us
        if score > 22 {
            score = 22;
        }
        let is_soft = HashRange::new(2, match hand.is_soft() {
            true => 1,
            false => 0,
        });

        // TODO: Include rules.automatic_win_at_hand_length()
        // Number of cards in the hand only matter if it's 1, 2, or 3+
        let cards_in_hand_hash = HashRange::new(3, {
            if hand.len() == 1 {
                0u
            } else if hand.len() == 2 {
                1u
            } else {
                2u
            }
        });
        let double_count = HashRange::new(
            rules.max_doubles_single_hand() + 1,
            hand.double_count());

        let splits_done = HashRange::new(
            rules.split_limit() + 1,
            hand.splits_done());

        let splits_to_do = HashRange::new(
            rules.split_limit() + 1,
            hand.splits_to_solve());

        create_hash([
                    HashRange::new(23, score),
                    is_soft,
                    splits_done,
                    cards_in_hand_hash,
                    double_count,
                    splits_to_do,
                    ])

    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use hand_hasher::create_hash;
    use hand_hasher::HashRange;
    use rules::BJRules;
    use shoe::randomshoe::new_infinite_shoe;
    use hand_hasher::DealerHandHasher;
    use hand_hasher::HandHasher;
    use hand::BJHand;
    use cards::value;
    use cards::value::Value;

    #[test]
    fn test_create_hash() {
        assert_eq!(
            vec![128],
            create_hash([HashRange::new(2, 1)]));
        assert_eq!(
            vec![128],
            create_hash([HashRange::new(4, 1)]));
        assert_eq!(
            vec![128],
            create_hash([HashRange::new(128, 1)]));
        assert_eq!(
            vec![128],
            create_hash([HashRange::new(256, 1)]));
        // At this boundary, we require two bytes
        assert_eq!(
            vec![128, 0],
            create_hash([HashRange::new(257, 1)]));
        // At this boundary, we require two bytes
        assert_eq!(
            vec![0, 128],
            create_hash([HashRange::new(257, 256)]));

        assert_eq!(
            vec![0],
            create_hash([
                        HashRange::new(2, 0),
                        HashRange::new(2, 0)
                        ]));
        assert_eq!(
            vec![0],
            create_hash([
                        HashRange::new(2, 0),
                        HashRange::new(128, 0)
                        ]));
        assert_eq!(
            vec![0, 0],
            create_hash([
                        HashRange::new(2, 0),
                        HashRange::new(128, 0),
                        HashRange::new(2, 0),
                        ]));
        assert_eq!(
            vec![255],
            create_hash([
                        HashRange::new(2, 1),
                        HashRange::new(16, 15),
                        HashRange::new(8, 7),
                        ]));
    }


    fn ensure_equal_values(hasher: &HandHasher, rules: BJRules, h1: Vec<Value>, h2: Vec<Value>) {
        let mut shoe = new_infinite_shoe();

        assert_eq!(
            hasher.hash_hand(&rules,
                             &BJHand::new_from_deck(&mut shoe, &h1).unwrap()),
            hasher.hash_hand(&rules,
                             &BJHand::new_from_deck(&mut shoe, &h2).unwrap()));
    }

    fn ensure_not_equal_values(hasher: &HandHasher, rules: BJRules, h1: Vec<Value>, h2: Vec<Value>) {
        let mut shoe = new_infinite_shoe();

        assert!(
            hasher.hash_hand(&rules,
                             &BJHand::new_from_deck(&mut shoe, &h1).unwrap()) !=
            hasher.hash_hand(&rules,
                             &BJHand::new_from_deck(&mut shoe, &h2).unwrap()));
    }

    #[test]
    fn test_dealer_hash() {
        let mut rules = BJRules::new();
        let hasher = &DealerHandHasher;

        ensure_equal_values(
            hasher,
            rules,
            vec![value::TEN, value::SIX],
            vec![value::EIGHT, value::EIGHT]);
        ensure_equal_values(
            hasher,
            rules,
            vec![value::ACE, value::SEVEN],
            vec![value::TEN, value::EIGHT]);
        ensure_equal_values(
            hasher,
            rules,
            vec![value::ACE, value::SIX],
            vec![value::TEN, value::SEVEN]);
        ensure_equal_values(
            hasher,
            rules,
            vec![value::ACE, value::SIX],
            vec![value::TEN, value::SEVEN]);
        ensure_not_equal_values(
            hasher,
            rules,
            vec![value::ACE, value::SEVEN],
            vec![value::TEN, value::SEVEN]);
        ensure_not_equal_values(
            hasher,
            rules,
            vec![value::ACE, value::THREE],
            vec![value::TEN, value::FOUR]);

        rules = BJRules::new_complex(false, 4, true, 1);

        ensure_not_equal_values(
            hasher,
            rules,
            vec![value::ACE, value::SIX],
            vec![value::TEN, value::SEVEN]);
        ensure_equal_values(
            hasher,
            rules,
            vec![value::ACE, value::SEVEN],
            vec![value::TEN, value::EIGHT]);
    }
}
