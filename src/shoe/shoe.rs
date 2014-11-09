use cards::value::Value;
use cards::value;
use cards::card::CardImpl;
use cards::card::Card;

pub trait DirectShoe {
    fn pop(&mut self) -> Option<CardImpl>;
    fn len(&self) -> uint;
    fn count(&self, v: &Value) -> uint;
    fn remove(&mut self, v: &Value) -> Option<CardImpl>;
    fn insert(&mut self, v: &Card);
}

// TODO: define only for test?
pub fn test_single_deck(dsInput: &mut DirectShoe) {
    use cards::card::Card;
    use cards::value::Value;
    use cardshoetracker;
    use cardshoetracker::CardShoeTracker;
    let ds = &mut cardshoetracker::CardShoeTracker::new(dsInput);
    // TODO: Track suit as well
    let mut num_4 = 0i;
    assert_eq!(52, ds.len());
    let mut count = 0i;
    while ds.len() > 0 {
        let item = ds.pop();
        match item {
            Some(c) => 
            {
                count += 1;
                // TODO: Impl equal for a trait?
                if c.value().desc() == value::FOUR.desc() {
                    num_4 += 1;
                }
            }
            None =>
                break
        }
    }
    assert_eq!(4, num_4);
    assert_eq!(4, ds.countValue(value::FOUR));
    assert_eq!(52, count);
    assert_eq!(52, ds.seenCards());
}
