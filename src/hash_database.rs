use std::collections::HashMap;

pub trait HashDatabase {
    fn get(&self, hash: Vec<u8>) -> Option<f64>;
    fn store(&mut self, hash: Vec<u8>, value: f64) -> Option<f64>;
}

pub struct InMemoryHashDatabase {
    db: HashMap<Vec<u8>, f64>,
}

impl InMemoryHashDatabase {
    pub fn new() -> InMemoryHashDatabase {
        InMemoryHashDatabase {
            db: HashMap::new(),
        }
    }
}

impl HashDatabase for InMemoryHashDatabase {
    fn get(&self, hash: Vec<u8>) -> Option<f64> {
        match self.db.get(&hash) {
            Some(s) => Some(*s),
            None => None,
        }
    }
    fn store(&mut self, hash: Vec<u8>, value: f64) -> Option<f64> {
        self.db.insert(hash, value)
    }
}


#[deriving(Copy)]
pub struct NoOpDatabase;
impl HashDatabase for NoOpDatabase {
    fn get(&self, _: Vec<u8>) -> Option<f64> {
        None
    }
    fn store(&mut self, _: Vec<u8>, _: f64) -> Option<f64> {
        None
    }
}


#[cfg(test)]
mod tests {
    extern crate test;
    use hash_database::InMemoryHashDatabase;
    use hash_database::HashDatabase;
    #[test]
    fn test_in_memory() {
        let mut m = InMemoryHashDatabase::new();
        assert_eq!(None, m.store(vec![1], 10.0));
        assert_eq!(None, m.store(vec![2], 20.0));
        assert_eq!(Some(10.0), m.get(vec![1]));
        assert_eq!(Some(10.0), m.store(vec![1], 30.0));
        assert_eq!(Some(30.0), m.get(vec![1]));
    }
}
