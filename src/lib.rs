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

pub fn slice_entropy(input: &[u8]) -> f64 {
    let mut c = Calculator::new();

    c.write_all(input)
        .expect("Writing bytes to the calculator cannot fail");

    c.entropy()
}

pub mod colour;

#[cfg(test)]
mod test {
    use crate::slice_entropy;

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
