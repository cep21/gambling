use cards;
use value;

pub trait DirectShoe {
    fn pop(&mut self) -> Option<cards::CardImpl>;
    fn len(&self) -> uint;
    fn count(&self, v: &value::Value) -> uint;
}

// TODO: define only for test?
pub fn test_single_deck(dsInput: &mut DirectShoe) {
    use cards::Card;
    use value::Value;
    use cardshoetracker;
    use cardshoetracker::CardShoeTracker;
    use value;
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
