# tropy - command line entropy viewer
[![Latest Version](https://img.shields.io/crates/v/tropy.svg)](https://crates.io/crates/tropy)
[![Documentation](https://docs.rs/tropy/badge.svg)](https://docs.rs/crate/tropy)
![License](https://img.shields.io/crates/l/tropy.svg)

## What is tropy?

The tropy commandline tool takes bytes or input from std in and calculates the [Shannon entropy](https://en.wikipedia.org/wiki/Entropy_(information_theory)) then it display them colour coded in the terminal.

## Installing
With [Rust installed](https://rustup.rs) run:
```shell
cargo install tropy
``` 

## Using tropy as a library
Add this to your ```Cargo.toml```:
```toml
...
#[dependencies]
...
tropy = { version = "0.1.0", default_features = false }
...
```
And to use the entropy calculator in your program/library add this:
```rust
use std::io::Write;
use tropy::Calculator;

let mut calc = Calculator::new();

// write some bytes
calc.write(&[0u8]).unwrap();
calc.write(&[1u8]).unwrap();

// calculate the entropy over the accumulated state
// and reset the calculator
let e = calc.entropy();
assert_eq!(e,1.0);

```

# License
[Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0)