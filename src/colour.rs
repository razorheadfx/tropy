use std::convert::From;
use std::fmt;
use std::fmt::{Display, Formatter};

extern crate hsl;

/// Colour represented as HSL
pub use hsl::HSL as Hsl;

/// Colour represented as RGB
#[derive(Debug, Clone, PartialEq)]
pub struct Rgb(pub u8, pub u8, pub u8);

/// A coloured [str] (either the character, background or both) when printed to a terminal which supports ANSI escape sequences
/// and 24bit colour codes.
///
/// See [https://gist.github.com/XVilka/8346728](https://gist.github.com/XVilka/8346728) for a non-comprehensive list of such terminals.
#[derive(Debug, Clone)]
pub enum RGB<'a> {
    /// Coloured Foreground
    Fg(Rgb, &'a str),
    /// Coloured Background
    Bg(Rgb, &'a str),
    /// Coloured Foreground and Background
    FgBg(Rgb, Rgb, &'a str),
}

impl Rgb {
    /// Set the foreground colour to the given RGB colour
    pub fn fg<'a>(&self, txt: &'a str) -> RGB<'a> {
        RGB::Fg(self.clone(), txt)
    }

    /// Set the background colour to the given RGB colour
    pub fn bg<'a>(&self, txt: &'a str) -> RGB<'a> {
        RGB::Bg(self.clone(), txt)
    }

    /// Set the foreground and background colours to the given RGB colours
    pub fn fgbg<'a>(&self, bg_col: &Rgb, txt: &'a str) -> RGB<'a> {
        RGB::FgBg(self.clone(), bg_col.clone(), txt)
    }
}

impl<'a> Display for RGB<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            RGB::Fg(c, txt) => write!(f, "\x1B[38;2;{};{};{}m{}\x1B[39m", c.0, c.1, c.2, txt),
            RGB::Bg(c, txt) => write!(f, "\x1B[48;2;{};{};{}m{}\x1B[49m", c.0, c.1, c.2, txt),
            RGB::FgBg(fg, bg, txt) => write!(
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
