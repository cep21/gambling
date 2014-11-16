use cards::color;

pub struct Suit {
    desc: &'static str,
    c: &'static color::Color,
    i: uint,
}

impl Suit {
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

pub const SPADE: Suit = Suit{desc:"spade", c: &color::BLACK, i: 0};
pub const CLUB: Suit = Suit{desc:"club", c: &color::BLACK, i: 1};
pub const HEART: Suit = Suit{desc:"heart", c: &color::RED, i:2};
pub const DIAMOND: Suit = Suit{desc:"diamond", c: &color::RED, i:3};

pub const SUITS: [Suit, ..4] = [SPADE, CLUB, HEART, DIAMOND];

#[test]
fn test_suits() {
    assert_eq!("spade", SPADE.desc());
    assert_eq!("black", SPADE.color().desc());
    assert_eq!("club", CLUB.desc());
    assert_eq!("heart", HEART.desc());
    assert_eq!("diamond", DIAMOND.desc());
    assert_eq!(0, SPADE.index());
}
