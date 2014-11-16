pub struct Value {
    desc: &'static str,
    i: uint,
}

impl Value {
    pub fn desc(&self) -> &str {
        return self.desc;
    }
    pub fn index(&self) -> uint {
        return self.i;
    }
}

pub const ACE: Value = Value{desc:"ace", i:0};
pub const TWO: Value = Value{desc:"two", i:1};
pub const THREE: Value = Value{desc:"three", i:2};
pub const FOUR: Value = Value{desc:"four", i:3};
pub const FIVE: Value = Value{desc:"five", i:4};
pub const SIX: Value = Value{desc:"six", i:5};
pub const SEVEN: Value = Value{desc:"seven", i:6};
pub const EIGHT: Value = Value{desc:"eight", i:7};
pub const NINE: Value = Value{desc:"nine", i:8};
pub const TEN: Value = Value{desc:"ten", i:9};
pub const JACK: Value = Value{desc:"jack", i:10};
pub const QUEEN: Value = Value{desc:"queen", i:11};
pub const KING: Value = Value{desc:"king", i:12};
pub const JOKER: Value = Value{desc:"joker", i:13};

pub const VALUES: [Value, ..13] = [ACE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, JACK, QUEEN, KING];

#[test]
fn test_suits() {
    assert_eq!("ace", ACE.desc());
    assert_eq!("two", TWO.desc());
    assert_eq!("three", THREE.desc());
    assert_eq!("four", FOUR.desc());
    assert_eq!("five", FIVE.desc());
    assert_eq!("six", SIX.desc());
    assert_eq!("seven", SEVEN.desc());
    assert_eq!("eight", EIGHT.desc());
    assert_eq!("nine", NINE.desc());
    assert_eq!("ten", TEN.desc());
    assert_eq!("jack", JACK.desc());
    assert_eq!("queen", QUEEN.desc());
    assert_eq!("king", KING.desc());
    assert_eq!(12, KING.index());
}
