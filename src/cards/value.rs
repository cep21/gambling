pub trait Value {
    fn desc(&self) -> &str;
    fn index(&self) -> uint;
}

pub struct ValueImpl {
    desc: &'static str,
    i: uint,
}

impl Value for ValueImpl {
    fn desc(&self) -> &str {
        return self.desc;
    }
    fn index(&self) -> uint {
        return self.i;
    }
}

pub const ACE: ValueImpl = ValueImpl{desc:"ace", i:0};
pub const TWO: ValueImpl = ValueImpl{desc:"two", i:1};
pub const THREE: ValueImpl = ValueImpl{desc:"three", i:2};
pub const FOUR: ValueImpl = ValueImpl{desc:"four", i:3};
pub const FIVE: ValueImpl = ValueImpl{desc:"five", i:4};
pub const SIX: ValueImpl = ValueImpl{desc:"six", i:5};
pub const SEVEN: ValueImpl = ValueImpl{desc:"seven", i:6};
pub const EIGHT: ValueImpl = ValueImpl{desc:"eight", i:7};
pub const NINE: ValueImpl = ValueImpl{desc:"nine", i:8};
pub const TEN: ValueImpl = ValueImpl{desc:"ten", i:9};
pub const JACK: ValueImpl = ValueImpl{desc:"jack", i:10};
pub const QUEEN: ValueImpl = ValueImpl{desc:"queen", i:11};
pub const KING: ValueImpl = ValueImpl{desc:"king", i:12};
pub const JOKER: ValueImpl = ValueImpl{desc:"joker", i:13};

pub const VALUES: [ValueImpl, ..13] = [ACE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE, TEN, JACK, QUEEN, KING];

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
