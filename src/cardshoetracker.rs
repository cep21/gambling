use shoe;
use cards::value::Value;
use cards::value;
use cards::card::Card;
use shoe::shoe::DirectShoe;

pub struct CardShoeTracker<'a> {
    counts: [[uint, ..4], ..13],
    wrapping: &'a mut DirectShoe<'a> + 'a,
}

impl <'a>shoe::shoe::DirectShoe<'a> for CardShoeTracker<'a> {
    fn pop(&mut self) -> Option<Card<'a>> {
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
    fn count(&self, v: &Value) -> uint {
        return self.wrapping.count(v);
    }
    fn remove(&mut self, v: &Value) -> Option<Card<'a>> {
        return self.wrapping.remove(v);
    }
    fn insert(&mut self, v: &Card) {
        self.wrapping.insert(v);
    }
}

impl <'a>CardShoeTracker<'a> {
    /*
       TODO: Why does new not work?
    pub fn new(wrap: &'a mut DirectShoe) -> CardShoeTracker<'a> {
        return CardShoeTracker {
            counts: [[0, 0, 0, 0], ..13],
            wrapping: wrap,
        };
    }
    */
    pub fn countValue(&self, ref v: Value) -> uint {
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
