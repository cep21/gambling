use cards::color;
use std::fmt;

#[derive(Clone, Copy)]
pub struct Suit {
    desc: &'static str,
    c: &'static color::Color,
    i: usize,
}

impl Suit {
    pub fn desc(&self) -> &str {
        return self.desc;
    }
    pub fn color(&self) -> &color::Color {
        return self.c;
    }
    pub fn index(&self) -> usize {
        return self.i;
    }
    pub fn char(&self) -> char {
        match self.desc.chars().next() {
            Some(c) => c,
            None => '?',
        }
    }
}

impl fmt::Show for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.desc())
    }
}

impl PartialEq for Suit {
    fn eq(&self, other: &Suit) -> bool {
        self.desc == other.desc
    }
}

pub const SPADE: Suit = Suit{desc:"spade", c: &color::BLACK, i: 0};
pub const CLUB: Suit = Suit{desc:"club", c: &color::BLACK, i: 1};
pub const HEART: Suit = Suit{desc:"heart", c: &color::RED, i:2};
pub const DIAMOND: Suit = Suit{desc:"diamond", c: &color::RED, i:3};

pub const SUITS: [Suit;4] = [SPADE, CLUB, HEART, DIAMOND];

#[test]
fn test_suits() {
    assert_eq!("spade", SPADE.desc());
    assert_eq!("black", SPADE.color().desc());
    assert_eq!("club", CLUB.desc());
    assert_eq!("heart", HEART.desc());
    assert_eq!("diamond", DIAMOND.desc());
    assert_eq!(0, SPADE.index());
    assert_eq!(SPADE, SPADE);
    println!("{:?}", SPADE);
}
