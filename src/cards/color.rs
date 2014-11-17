pub struct Color {
    desc: &'static str,
}

impl Color {
    pub fn desc(&self) -> &str {
        return self.desc
    }
}

pub const RED: Color= Color{desc:"red"};
pub const BLACK: Color= Color{desc:"black"};

pub const COLORS: [Color, ..2] = [BLACK, RED];

#[test]
fn test_colors() {
    assert_eq!("red", "red");
    assert_eq!(RED.desc(), RED.desc());
    assert_eq!("red", RED.desc());
    assert_eq!("black", BLACK.desc());
}
