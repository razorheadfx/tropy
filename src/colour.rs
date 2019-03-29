use std::convert::From;
use std::fmt;
use std::fmt::{Display, Formatter};

extern crate hsl;

pub use hsl::HSL as Hsl;

/// RGB
#[derive(Debug, Clone, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

#[derive(Debug, Clone)]
pub enum RGB<'a> {
    Fg(Rgb, &'a str),
    Bg(Rgb, &'a str),
    Both(Rgb, Rgb, &'a str),
}

impl Rgb {
    pub fn fg<'a>(&self, txt: &'a str) -> RGB<'a> {
        RGB::Fg(self.clone(), txt)
    }

    pub fn bg<'a>(&self, txt: &'a str) -> RGB<'a> {
        RGB::Bg(self.clone(), txt)
    }

    pub fn both<'a>(&self, bg_col: Rgb, txt: &'a str) -> RGB<'a> {
        RGB::Both(self.clone(), bg_col, txt)
    }
}

impl<'a> Display for RGB<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RGB::Fg(c, txt) => write!(f, "\x1B[38;2;{};{};{}m{}\x1B[39m", c.0, c.1, c.2, txt),
            RGB::Bg(c, txt) => write!(f, "\x1B[48;2;{};{};{}m{}\x1B[49m", c.0, c.1, c.2, txt),
            RGB::Both(fg, bg, txt) => write!(
                f,
                "\x1B[38;2;{};{};{}m\x1B[48;2;{};{};{}m{}\x1B[49m\x1B[39m",
                fg.0, fg.1, fg.2, bg.0, bg.1, bg.2, txt
            ),
        }
    }
}

impl From<Hsl> for Rgb {
    fn from(x: Hsl) -> Self {
        let (r, g, b) = x.to_rgb();

        Rgb(r, g, b)
    }
}

#[cfg(test)]
mod test {
    use crate::colour::{Rgb, RGB};
    use std::io::Write;

    #[test]
    fn fg_rgb() {
        let s = "\x1B[38;2;255;255;255mblaaa\x1B[39m";
        let mut su = vec![];
        write!(su, "{}", RGB::Fg(Rgb(255, 255, 255), "blaaa")).unwrap();
        assert_eq!(s, String::from_utf8(su).unwrap());
    }

}
