use cards::color;
pub trait Suit {
    fn color(&self) -> &color::Color;
    fn desc(&self) -> &str;
    fn index(&self) -> uint;
}

pub struct SuitImpl {
    desc: &'static str,
    c: &'static color::ColorImpl,
    i: uint,
}

impl Suit for SuitImpl {
    fn desc(&self) -> &str {
        return self.desc;
    }
    fn color(&self) -> &color::Color {
        return self.c;
    }
    fn index(&self) -> uint {
        return self.i;
    }
}

pub const SPADE: SuitImpl = SuitImpl{desc:"spade", c: &color::BLACK, i: 0};
pub const CLUB: SuitImpl = SuitImpl{desc:"club", c: &color::BLACK, i: 1};
pub const HEART: SuitImpl = SuitImpl{desc:"heart", c: &color::RED, i:2};
pub const DIAMOND: SuitImpl = SuitImpl{desc:"diamond", c: &color::RED, i:3};

pub const SUITS: [SuitImpl, ..4] = [SPADE, CLUB, HEART, DIAMOND];

#[test]
fn test_suits() {
    assert_eq!("spade", SPADE.desc());
    assert_eq!("black", SPADE.color().desc());
    assert_eq!("club", CLUB.desc());
    assert_eq!("heart", HEART.desc());
    assert_eq!("diamond", DIAMOND.desc());
    assert_eq!(0, SPADE.index());
}
