use std::io;
use std::io::Write;

/// An entropy calculator
///
/// # Example
///```
/// // we need Write because that is how values are collected for calculation
/// use std::io::Write;
/// use tropy::Calculator;
///
/// let mut c = Calculator::new();
/// // write some bytes
/// c.write(&[0u8]).unwrap();
/// c.write(&[1u8]).unwrap();
///
/// // calculate the entropy over the accumulated state
/// let e = c.entropy();
/// assert_eq!(e,1.0);
///
///```
pub struct Calculator {
    // this is 2kB (256*8bytes)in size so it might blow the stack for certain configurations
    // hence boxed
    counts: Box<[u64; 256]>,
    nbytes: usize,
}

impl Calculator {
    /// Instantiate a new calculator.
    pub fn new() -> Self {
        Calculator {
            counts: Box::new([0u64; 256]),
            nbytes: 0,
        }
    }

    /// Calculate Shannon entropy over the bytes written so far and clear the calculator's internal state.
    pub fn entropy(&mut self) -> f64 {
        let bytes = self.nbytes as f64;
        let e = self
            .counts
            .iter()
            .cloned()
            .filter(|c| c > &0u64)
            .map(|c| c as f64 / bytes)
            .map(|p| p * p.log2())
            .fold(0.0, |h, x| h - x);

        self.counts.iter_mut().for_each(|c| *c = 0u64);
        self.nbytes = 0;
        e
    }
}

impl Write for Calculator {
    fn write(&mut self, input: &[u8]) -> io::Result<usize> {
        input.iter().for_each(|byte| {
            self.counts[*byte as usize] = self.counts[*byte as usize]
                .checked_add(1)
                .expect("Count exceeded the length of a u64; Where'd you get that many bytes from?")
        });
        let bytes = input.len();
        self.nbytes += bytes;
        Ok(bytes)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Convenience method: Creates an [Calculator] fills it with the given input and returns the
/// calculated entropy.
pub fn slice_entropy(input: &[u8]) -> f64 {
    let mut c = Calculator::new();

    c.write_all(input)
        .expect("Writing bytes to the calculator cannot fail");

    c.entropy()
}

/// Print coloured output using ANSI escape sequences.
/// The terminal in use must support it.
///
/// HSL support is provided by [hsl](https://crates.io/crates/hsl)
/// # Example
/// ```
/// use tropy::colour::{Rgb,Hsl};
/// // RGB is not used directly but via the Rgb struct
/// let red = Rgb(255,0,0);
/// let green = Rgb(0,255,0);
/// println!("{}", red.fg("This foreground is red"));
/// println!("{}", green.bg("This background is green"));
/// println!("{}", red.fgbg(&green, "The foreground is red and the background is green. Lovely"));
/// let some_hsl_colour = Hsl{h: 90.0, s : 1.0, l : 0.5};
/// println!("{}", Rgb::from(some_hsl_colour).fg("This is some HSL colour"));
///
/// ```
pub mod colour;

#[cfg(test)]
mod test {
    use crate::slice_entropy;

    #[test]
    fn simple() {
        let x = [0u8, 23u8, 66u8, 1u8];
        let e = slice_entropy(&x);

        assert_eq!(e, 2.0);

        let x = [0u8, 0u8, 66u8, 1u8];
        let e = slice_entropy(&x);
        assert_eq!(e, 1.5);

        let x = [0u8, 1u8];
        let e = slice_entropy(&x);
        assert_eq!(e, 1.0);
    }
}
