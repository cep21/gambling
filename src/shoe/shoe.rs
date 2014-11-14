use cards::value::Value;
use cards::value::ValueImpl;
use cards::value;
use cards::card::CardImpl;
use cards::card::Card;

pub trait DirectShoe {
    fn pop(&mut self) -> Option<CardImpl>;
    fn len(&self) -> uint;
    fn count(&self, v: &Value) -> uint;
    fn remove(&mut self, v: &ValueImpl) -> Option<CardImpl>;
    fn insert(&mut self, v: &Card);
}

// TODO: define only for test?
pub fn test_single_deck(ds: &mut DirectShoe) {
    use cards::card::Card;
    use cards::value::Value;
    use cards::value::ACE;
    use std::collections::HashSet;
    assert_eq!(52, ds.len());

    assert_eq!(4, ds.count(&ACE));
    {
        let mut ace_suit_tracking_set = HashSet::new();
        // Make sure there are 4 aces
        for _ in range(0, 4u) {
            match ds.remove(&ACE) {
                Some(c) => {
                    ace_suit_tracking_set.insert(c.suit().index());
                    assert_eq!(c.value().index(), ACE.index())
                },
                _ => {}
            }
        }
        assert_eq!(4, ace_suit_tracking_set.len());
        match ds.remove(&ACE) {
            Some(_) => fail!("Expect none!"),
            _ => {}
        }
    }
    assert_eq!(0, ds.count(&ACE));

    {
        let c = ds.remove(&value::FOUR);
        match c {
            Some(c) => {
                assert_eq!(52 - 5, ds.len());
                assert_eq!(value::FOUR.index(), c.value().index());
                assert_eq!(3, ds.count(&value::FOUR));
                ds.insert(&c);
                assert_eq!(4, ds.count(&value::FOUR));
            }
            None => {
                fail!("Should be able to remove a four")
            }
        }
    }
    println!("Looping");
    let mut num_4 = 0i;
    let mut count = 0i;
    let mut suit_tracking_count = HashSet::new();
    let value_to_track = value::FIVE;
    assert_eq!(4, ds.count(&value_to_track));
    assert_eq!(52 - 4, ds.len());

    while ds.len() > 0 {
        println!("Starting pop");
        let item = ds.pop();
        match item {
            Some(c) => 
            {
                count += 1;
                // TODO: Impl equal for a trait?
                if c.value().desc() == value_to_track.desc() {
                    num_4 += 1;
                    suit_tracking_count.insert(c.suit().index());
                }
            }
            None =>
                break
        }
    }
    assert_eq!(0, ds.count(&value_to_track));
    assert_eq!(52 - 4, count);
    assert_eq!(4, num_4);
    assert_eq!(4, suit_tracking_count.len());
}
