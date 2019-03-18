use std::io;
use std::io::Write;

pub struct Calculator {
    // this is 16kB in size so it might blow the stack for certain configurations
    // hence boxed
    counts: Box<[u64; 256]>,
    nbytes: usize,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            counts: Box::new([0u64; 256]),
            nbytes: 0,
        }
    }

    /// calculates Shannon entropy over the bytes written so far
    /// and clears the internal state
    pub fn entropy(&mut self) -> f64 {
        let bytes = self.nbytes as f64;
        let e = self
            .counts
            .iter()
            .filter(|c| *c > &0u64)
            .map(|c| *c as f64 / bytes)
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

pub fn slice_entropy(input: &[u8]) -> f64 {
    let mut c = Calculator::new();

    c.write(input).expect("This should never fail");

    c.entropy()
}

pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    // hsl to rgb conversion in the case i drop colorful in favour of termion
    assert!(0.0 <= h && h <= 1.0);
    assert!(0.0 <= s && s <= 1.0);
    assert!(0.0 <= l && l <= 1.0);

    // calc chroma
    if s == 0.0 {
        let g = (255.0 * l) as u8;
        (g, g, g)
    } else {
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - (h / 6.0 % 2.0 - 1.0).abs());
        let m = l - c / 2.0;
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use crate::{hsl_to_rgb, slice_entropy};
    #[test]
    fn hsl() {
        assert_eq!(hsl_to_rgb(0.0, 1.0, 0.5), (255, 0, 0));
    }

    #[test]
    fn calculate_entropy() {
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
