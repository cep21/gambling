use std::fmt;
pub struct Color {
    desc: &'static str,
}

impl Copy for Color {}

impl Color {
    pub fn desc(&self) -> &str {
        return self.desc
    }
}

impl fmt::Show for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.desc())
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.desc == other.desc
    }
}


pub const RED: Color= Color{desc:"red"};
pub const BLACK: Color= Color{desc:"black"};

pub const COLORS: [Color;2] = [BLACK, RED];

#[test]
fn test_colors() {
    assert_eq!("red", "red");
    assert_eq!(RED.desc(), RED.desc());
    assert_eq!(RED, RED);
    assert_eq!("red", RED.desc());
    assert_eq!("black", BLACK.desc());
    println!("{:?}", RED);
}
