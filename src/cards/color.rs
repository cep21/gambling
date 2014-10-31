pub trait Color {
    fn desc(&self) -> &str;
}

pub struct ColorImpl {
    desc: &'static str,
}

impl Color for ColorImpl {
    fn desc(&self) -> &str {
        return self.desc
    }
}

pub const RED: ColorImpl = ColorImpl{desc:"red"};
pub const BLACK: ColorImpl = ColorImpl{desc:"black"};

pub const COLORS: [ColorImpl, ..2] = [BLACK, RED];

#[test]
fn test_colors() {
    assert_eq!("red", "red");
    assert_eq!(RED.desc(), RED.desc());
    assert_eq!("red", RED.desc());
    assert_eq!("black", BLACK.desc());
}
