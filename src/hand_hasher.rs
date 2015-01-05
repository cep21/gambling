extern crate num;
use rules::BJRules;
use bjaction::BJAction::HIT;
use bjaction::BJAction::STAND;
use bjaction::BJAction::DOUBLE;
use bjaction::BJAction::SURRENDER;
use bjaction::BJAction::SPLIT;
use cards::value::VALUES;
use hand::BJHand;
use std::collections::Bitv;
use std::num::Int;
use shoe::shoe::DirectShoe;
use self::num::bigint::BigUint;
use self::num::Zero;
use self::num::Integer;
use self::num::bigint::ToBigUint;

pub trait HandHasher {
    fn hash_hand(&self, rules: &BJRules, hand: &BJHand) -> Vec<u8>;
    fn hash_hand_ignore_actions(&self, rules: &BJRules, hand: &BJHand) -> Vec<u8>;
}

pub trait DeckHasher {
    fn hash_deck(&self, rules: &BJRules, shoe: &DirectShoe) -> Vec<u8>;
}

#[deriving(Copy)]
pub struct DealerHandHasher;

struct HashRange {
    // Should be inclusive (so 1 means 2 values (0 or 1))
    max_value: uint,
    // Should be < max_value
    current_value: uint,
}

//static biguintCache: Vec<int> = range(0, 10).map(|m| m).collect();

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

enum DynamicChangingU64 {
    Regular(u64),
    Struct(BigUint),
}

impl DynamicChangingU64 {
    fn new(val: u64) -> DynamicChangingU64 {
        DynamicChangingU64::Regular(val)
    }
    fn is_zero(&self) -> bool {
        match *self {
            DynamicChangingU64::Regular(ref v) => {
                *v == 0
            }
            DynamicChangingU64::Struct(ref v) => {
                v.is_zero()
            }
        }
    }
    fn is_odd(&self) -> bool {
        match *self {
            DynamicChangingU64::Regular(ref v) => {
                *v % 2 == 1
            }
            DynamicChangingU64::Struct(ref v) => {
                v.is_odd()
            }
        }
    }
    fn bits(&self) -> uint {
        match *self {
            DynamicChangingU64::Regular(ref v) => {
                v.to_biguint().unwrap().bits()
            }
            DynamicChangingU64::Struct(ref v) => {
                v.bits()
            }
        }
    }
}

impl Add<uint, DynamicChangingU64> for DynamicChangingU64 {
    fn add(self, other: uint) -> DynamicChangingU64 {
        match self {
            DynamicChangingU64::Regular(ref v) => {
                match v.checked_add(other as u64) {
                    Some(v2) => DynamicChangingU64::Regular(v2),
                    None => DynamicChangingU64::Struct(v.to_biguint().unwrap() +
                                                       other.to_biguint().unwrap())
                }
            }
            DynamicChangingU64::Struct(ref v) => {
                DynamicChangingU64::Struct(v + other.to_biguint().unwrap())
            }
        }
    }
}

impl Mul<uint, DynamicChangingU64> for DynamicChangingU64 {
    fn mul(self, other: uint) -> DynamicChangingU64 {
        match self {
            DynamicChangingU64::Regular(ref v) => {
                match v.checked_mul(other as u64) {
                    Some(v2) => DynamicChangingU64::Regular(v2),
                    None => {
                        println!("switch over!");
                        DynamicChangingU64::Struct(v.to_biguint().unwrap() *
                                                       other.to_biguint().unwrap())
                    }
                }
            }
            DynamicChangingU64::Struct(ref v) => {
                DynamicChangingU64::Struct(v * other.to_biguint().unwrap())
            }
        }
    }
}

impl Shr<uint, DynamicChangingU64> for DynamicChangingU64 {
    fn shr(self, other: uint) -> DynamicChangingU64 {
        match self {
            DynamicChangingU64::Regular(ref v) => {
                DynamicChangingU64::Regular(*v >> other)
            }
            DynamicChangingU64::Struct(ref v) => {
                DynamicChangingU64::Struct(v >> other)
            }
        }
    }
}

fn create_hash(ranges: &[HashRange]) -> Vec<u8> {
    let mut val = DynamicChangingU64::new(0);
    let mut bits_required: DynamicChangingU64 = DynamicChangingU64::new(0);
    for i in ranges.iter() {
        // TODO: This could be a lot more efficient ...
        val = val * i.max_value;
        val = val + i.current_value;
        bits_required = bits_required * i.max_value +
            (i.max_value - 1);
    }
    let mut bv = Bitv::from_elem(bits_required.bits(), false);
    let mut current_bit = 0u;
    while !val.is_zero() {
        bv.set(current_bit, val.is_odd());
        current_bit += 1;
        val = val >> 1;
    }
    return bv.to_bytes();
}


impl HandHasher for DealerHandHasher {
    fn hash_hand(&self, rules: &BJRules, hand: &BJHand) -> Vec<u8> {
        let mut score = hand.score();
        // All scores > 22 are the same to us: Note may not be true for
        // push 22 rules.
        if score > 22 {
            score = 22;
        }
        let cards_in_hand_hash = HashRange::new(3, {
            if hand.len() == 1 {
                0u
            } else if hand.len() == 2 {
                1u
            } else {
                2u
            }
        });

        // Hash together the score and softness
        assert!(score <= 22);
        create_hash(&[
                    cards_in_hand_hash,
                    HashRange::new(23, score),
                    HashRange::new(2,
        // Treat soft 17 same as hard 17 if the dealer stands on both
                                   match hand.is_soft() &&
                                         rules.dealer_hits_soft_score(hand.score()) {
                                       true => 1,
                                       false => 0,
                                         })])
    }
    fn hash_hand_ignore_actions(&self, rules: &BJRules, hand: &BJHand) -> Vec<u8> {
        self.hash_hand(rules, hand)
    }
}

#[deriving(Copy)]
pub struct HandScoreHasher;

impl HandHasher for HandScoreHasher {
    fn hash_hand(&self, _: &BJRules, hand: &BJHand) -> Vec<u8> {
        let mut score = hand.score();
        // All scores > 22 are the same to us: Note may not be true for
        // push 22 rules.
        if score > 22 {
            score = 22;
        }

        // Hash together the score and softness
        assert!(score <= 22);
        create_hash(&[HashRange::new(23, score)])
    }
    fn hash_hand_ignore_actions(&self, rules: &BJRules, hand: &BJHand) -> Vec<u8> {
        self.hash_hand(rules, hand)
    }
}


#[deriving(Copy)]
pub struct PlayerHandHasher;

impl PlayerHandHasher {
    fn hash_impl(&self, rules: &BJRules, hand: &BJHand, include_actions: bool) -> Vec<u8> {
//        assert!(hand.len() >= 0);
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
        // Note: Number of cards in the hand actually don't matter, since it's
        //       covered by the rules:  For example, can_double(), etc may
        //       return different rules.  This allows games where you cannot
        //       double on 9 to treat 5+4 and 3+3+3 as the same hash.
/*        let cards_in_hand_hash = HashRange::new(3, {
            if hand.len() == 1 {
                0u
            } else if hand.len() == 2 {
                1u
            } else {
                2u
            }
        });*/

        let mut v = Vec::with_capacity(10);
        v.push(HashRange::new(23, score));
        v.push(is_soft);
        if rules.max_doubles_single_hand() > 0 {
            v.push(HashRange::new(
                rules.max_doubles_single_hand() + 1,
                hand.double_count())
            );
        }

        if rules.split_limit() > 0 {
            v.push(HashRange::new(
                rules.split_limit() + 1,
                hand.splits_done()));
            v.push(HashRange::new(
                rules.split_limit() + 1,
                hand.splits_to_solve()));
        }
        if include_actions {
            let actions = [STAND, HIT, DOUBLE, SPLIT, SURRENDER];
            for &action in actions.iter() {
                v.push(HashRange::new(2, match rules.can_take_action(hand, action) {
                    true => 1,
                    false => 0,
                }));
            }
        }

        create_hash(v.as_slice())
    }
}

impl HandHasher for PlayerHandHasher {
    fn hash_hand(&self, rules: &BJRules, hand: &BJHand) -> Vec<u8> {
        self.hash_impl(rules, hand, true)
    }
    fn hash_hand_ignore_actions(&self, rules: &BJRules, hand: &BJHand) -> Vec<u8> {
        self.hash_impl(rules, hand, false)
    }
}

#[deriving(Copy)]
pub struct SuitlessDeckHasher;

/**
 * Doesn't care about the suit of the cards in the deck
 */
impl DeckHasher for SuitlessDeckHasher {
    fn hash_deck(&self, _: &BJRules, shoe: &DirectShoe) -> Vec<u8> {
        match shoe.maximum_count_of_any_value() {
            None => vec![0],
            Some(s) => {
                let v: Vec<HashRange> = VALUES.iter().map(
                    |v| HashRange::new(s+1, shoe.count(v))).collect();
                create_hash(v.as_slice())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use hand_hasher::create_hash;
    use hand_hasher::HashRange;
    use rules::BJRules;
    use shoe::randomshoe::new_infinite_shoe;
    use self::test::Bencher;
    use hand_hasher::DealerHandHasher;
    use hand_hasher::PlayerHandHasher;
    use hand_hasher::HandHasher;
    use shoe::shoe::DirectShoe;
    use shoe::directshoe::DirectActualShoe;
    use hand_hasher::SuitlessDeckHasher;
    use shoe::deck::cards_in_deck;
    use hand_hasher::DeckHasher;
    use hand::BJHand;
    use cards::value;
    use cards::value::Value;

    #[test]
    fn test_create_hash() {
        assert_eq!(
            vec![128],
            create_hash(&[HashRange::new(2, 1)]));
        assert_eq!(
            vec![128],
            create_hash(&[HashRange::new(4, 1)]));
        assert_eq!(
            vec![128],
            create_hash(&[HashRange::new(128, 1)]));
        assert_eq!(
            vec![128],
            create_hash(&[HashRange::new(256, 1)]));
        // At this boundary, we require two bytes
        assert_eq!(
            vec![128, 0],
            create_hash(&[HashRange::new(257, 1)]));
        // At this boundary, we require two bytes
        assert_eq!(
            vec![0, 128],
            create_hash(&[HashRange::new(257, 256)]));

        assert_eq!(
            vec![0],
            create_hash(&[
                        HashRange::new(2, 0),
                        HashRange::new(2, 0)
                        ]));
        assert_eq!(
            vec![0],
            create_hash(&[
                        HashRange::new(2, 0),
                        HashRange::new(128, 0)
                        ]));
        assert_eq!(
            vec![0, 0],
            create_hash(&[
                        HashRange::new(2, 0),
                        HashRange::new(128, 0),
                        HashRange::new(2, 0),
                        ]));
        assert_eq!(
            vec![255],
            create_hash(&[
                        HashRange::new(2, 1),
                        HashRange::new(16, 15),
                        HashRange::new(8, 7),
                        ]));
    }


    fn ensure_equal_values(hasher: &HandHasher, rules: &BJRules, h1: Vec<Value>, h2: Vec<Value>) {
        let mut shoe = new_infinite_shoe();
        println!("Checking {} vs {}", h1, h2);

        assert_eq!(
            hasher.hash_hand(rules,
                             &BJHand::new_from_deck(&mut shoe, &h1).unwrap()),
            hasher.hash_hand(rules,
                             &BJHand::new_from_deck(&mut shoe, &h2).unwrap()));
    }

    fn ensure_not_equal_values(hasher: &HandHasher, rules: &BJRules, h1: Vec<Value>, h2: Vec<Value>) {
        let mut shoe = new_infinite_shoe();
        println!("Checking {} vs {}", h1, h2);

        assert!(
            hasher.hash_hand(rules,
                             &BJHand::new_from_deck(&mut shoe, &h1).unwrap()) !=
            hasher.hash_hand(rules,
                             &BJHand::new_from_deck(&mut shoe, &h2).unwrap()));
    }

    #[test]
    fn test_dealer_hash() {
        let rules = &BJRules::new();
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

        let rules2 = &BJRules::new_complex(false, 4, true, 1, false, false, false);

        ensure_not_equal_values(
            hasher,
            rules2,
            vec![value::ACE, value::SIX],
            vec![value::TEN, value::SEVEN]);
        ensure_equal_values(
            hasher,
            rules2,
            vec![value::ACE, value::SEVEN],
            vec![value::TEN, value::EIGHT]);
    }

    #[test]
    fn test_player_hash() {
        let rules = &BJRules::new();
        let hasher = &PlayerHandHasher;

        ensure_not_equal_values(
            hasher,
            rules,
            vec![value::TEN, value::SIX],
            // Not equal: You can split the 88
            vec![value::EIGHT, value::EIGHT]);

        ensure_equal_values(
            hasher,
            rules,
            vec![value::TEN, value::FOUR],
            vec![value::EIGHT, value::SIX]);

        ensure_equal_values(
            hasher,
            rules,
            vec![value::TEN, value::FOUR],
            vec![value::FOUR, value::TEN]);

        ensure_not_equal_values(
            hasher,
            rules,
            // Can't double/surrender the first hand
            vec![value::TEN, value::TWO, value::TWO],
            vec![value::FOUR, value::TEN]);

        ensure_not_equal_values(
            hasher,
            rules,
            // First hand is soft
            vec![value::ACE, value::SEVEN],
            vec![value::EIGHT, value::TEN]);

        // No surrender/split or double, so these two hands are the same
        let rules2 = &BJRules::new_complex(false, 0, true, 0, false, false, false);
        ensure_equal_values(
            hasher,
            rules2,
            vec![value::TEN, value::TWO, value::TWO],
            vec![value::FOUR, value::TEN]);

    }

    #[test]
    fn test_random_deck_hash() {
        let rules = BJRules::new();
        let hasher = SuitlessDeckHasher;
        let shoe = new_infinite_shoe();
        assert_eq!(
            hasher.hash_deck(&rules,
                             &shoe),
            hasher.hash_deck(&rules,
                             &shoe));
        let mut shoe2 = new_infinite_shoe();
        shoe2.pop().unwrap();
        assert_eq!(
            hasher.hash_deck(&rules,
                             &shoe),
            hasher.hash_deck(&rules,
                             &shoe2));
    }

    #[test]
    fn test_direct_deck_hash1() {
        let rules = BJRules::new();
        let hasher = SuitlessDeckHasher;
        let shoe1 = DirectActualShoe {
            cards: &mut cards_in_deck(1),
            initial_length: Some(52),
            maximum_count_of_any_value: Some(4),
        };
        let mut shoe2 = DirectActualShoe {
            cards: &mut cards_in_deck(1),
            initial_length: Some(52),
            maximum_count_of_any_value: Some(4),
        };
        assert_eq!(
            hasher.hash_deck(&rules,
                             &shoe1),
            hasher.hash_deck(&rules,
                             &shoe2));
        shoe2.pop().unwrap();
        assert!(
            hasher.hash_deck(&rules,
                             &shoe1) !=
            hasher.hash_deck(&rules,
                             &shoe2));
    }

    #[test]
    fn test_direct_deck_hash2() {
        let rules = BJRules::new();
        let hasher = SuitlessDeckHasher;
        let mut shoe3 = DirectActualShoe {
            cards: &mut cards_in_deck(1),
            initial_length: Some(52),
            maximum_count_of_any_value: Some(4),
        };
        let mut shoe4 = DirectActualShoe {
            cards: &mut cards_in_deck(1),
            initial_length: Some(52),
            maximum_count_of_any_value: Some(4),
        };

        shoe3.remove(&value::TEN);
        assert!(
            hasher.hash_deck(&rules,
                             &shoe3) !=
            hasher.hash_deck(&rules,
                             &shoe4));
        shoe4.remove(&value::FOUR);
        assert!(
            hasher.hash_deck(&rules,
                             &shoe3) !=
            hasher.hash_deck(&rules,
                             &shoe4));
        shoe3.remove(&value::FOUR);
        shoe4.remove(&value::TEN);
        assert_eq!(
            hasher.hash_deck(&rules,
                             &shoe3),
            hasher.hash_deck(&rules,
                             &shoe4));
    }

    #[bench]
    fn bench_create_hash(b: &mut Bencher) {
        let ranges = vec![
        HashRange::new(23, 21),
        HashRange::new(2, 0),
        HashRange::new(5, 1),
        HashRange::new(5, 2),
        HashRange::new(2, 0),
        HashRange::new(2, 0),
        HashRange::new(2, 1),
        HashRange::new(2, 1),
        HashRange::new(2, 1),
        HashRange::new(5, 2)];
        let range_slice = ranges.as_slice();
        let h1 = create_hash(range_slice);
        b.iter(|| {
            assert_eq!(h1, create_hash(range_slice));
        });
    }
}
