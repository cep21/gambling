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
    use cardshoetracker;
    use cardshoetracker::CardShoeTracker;
//    let ds = &mut cardshoetracker::CardShoeTracker::new(dsInput);
    // TODO: Track suit as well
    let mut num_4 = 0i;
    assert_eq!(52, ds.len());
    let mut count = 0i;
    let valueToTrack = value::FIVE;
    // Make sure there are 4 aces
    match ds.remove(&ACE) {
        Some(c) => assert_eq!(c.value().index(), ACE.index()),
        _ => {}
    }
    match ds.remove(&ACE) {
        Some(c) => assert_eq!(c.value().index(), ACE.index()),
        _ => {}
    }
    match ds.remove(&ACE) {
        Some(c) => assert_eq!(c.value().index(), ACE.index()),
        _ => {}
    }
    match ds.remove(&ACE) {
        Some(c) => assert_eq!(c.value().index(), ACE.index()),
        _ => {}
    }
    println!("Removing another ace");
    match ds.remove(&ACE) {
        Some(c) => fail!("Expect none!"),
        _ => {}
    }
    println!("Looping");
    while ds.len() > 0 {
        let item = ds.pop();
        println!("Got an item");
        match item {
            Some(c) => 
            {
                count += 1;
                // TODO: Impl equal for a trait?
                if c.value().desc() == valueToTrack.desc() {
                    num_4 += 1;
                }
            }
            None =>
                break
        }
    }
    assert_eq!(52, count);
//    assert_eq!(52 - 4, ds.seenCards());
    assert_eq!(4, num_4);
    assert_eq!(4, ds.count(&valueToTrack));
}
