use shoe;
use cards;
use value;
use value::Value;
use cards::Card;

pub struct CardShoeTracker<'a> {
    counts: [[uint, ..4], ..13],
    wrapping: &'a mut shoe::DirectShoe + 'a,
}

impl <'a>shoe::DirectShoe for CardShoeTracker<'a> {
    fn pop(&mut self) -> Option<cards::CardImpl> {
        let r = self.wrapping.pop();
        match r {
            Some(r) => self.counts[r.value().index()][r.suit().index()] += 1,
            None => (),
        };
        return r;
    }
    fn len(&self) -> uint {
        return self.wrapping.len();
    }
}

impl <'a>CardShoeTracker<'a> {
    pub fn new(wrap: &'a mut shoe::DirectShoe) -> CardShoeTracker<'a> {
        return CardShoeTracker{
            counts: [[0, 0, 0, 0], ..13],
            wrapping: wrap,
        };
    }
    pub fn countValue(&self, ref v: value::ValueImpl) -> uint {
        let mut ret = 0;
        for &i in self.counts[v.index()].iter() {
            ret += i;
        }
        return ret;
    }
    pub fn seenCards(&self) -> uint {
        let mut ret = 0;
        for &v in value::VALUES.iter() {
            ret += self.countValue(v);
        }
        return ret;
    }
}
