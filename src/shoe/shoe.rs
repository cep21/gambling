use cards::value::Value;
use cards::value::VALUES;
use cards::card::Card;

pub trait DirectShoe {
    fn pop(&mut self) -> Option<Card>;
    fn len(&self) -> uint;
    fn count(&self, v: &Value) -> uint;
    fn remove(&mut self, v: &Value) -> Option<Card>;
    fn insert(&mut self, v: &Card);
    // No initial length means the deck has no initial length: is infinite
    fn initial_length(&self) -> Option<uint>;
    // No count means the deck has no initial length: is infinite
    fn maximum_count_of_any_value(&self) -> Option<uint>;
}

pub fn fmt(d: &DirectShoe) -> String {
    let mut s = String::new();
    for v in VALUES.iter() {
        s.push_str(d.count(v).to_string().as_slice());
        s.push_str(v.desc());
        s.push_str(" ");
    }
    s.to_string()
}

/*
 * TODO: fmt::Show throws a compile exception
impl <'a>fmt::Show for DirectShoe + 'a {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hello")
    }
}
*/

// TODO: define only for test?
#[cfg(test)]
pub fn test_single_deck(ds: &mut DirectShoe) {
    use cards::value::ACE;
    use cards::value;
    use std::collections::HashSet;
    assert_eq!(52, ds.len());

    println!("Counting aces");
    assert_eq!(4, ds.count(&ACE));
    {
        let mut ace_suit_tracking_set = HashSet::new();
        // Make sure there are 4 aces
        for i in range(0, 4u) {
            println!("Removing one ace");
            match ds.remove(&ACE) {
                Some(c) => {
                    ace_suit_tracking_set.insert(c.suit().index());
                    assert_eq!(c.value().index(), ACE.index())
                },
                _ => {}
            }
            assert_eq!(3 - i, ds.count(&ACE));
        }
        assert_eq!(4, ace_suit_tracking_set.len());
        match ds.remove(&ACE) {
            Some(_) => panic!("Expect none!"),
            _ => {}
        }
    }
    assert_eq!(0, ds.count(&ACE));

    println!("Removing and inserting a four");
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
                panic!("Should be able to remove a four")
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
