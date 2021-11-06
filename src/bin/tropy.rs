extern crate structopt;

use structopt::StructOpt;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::exit;
use tropy::colour::{Hsl, Rgb};
use tropy::Calculator;

/// Read bytes from file or stdin and calculate the Shannon entropy for for chunks of a fixed size.
/// Then display it colour-coded in the terminal or write it to stdout as csv.
#[derive(Debug, StructOpt)]
struct Tropy {
    #[structopt(
        name = "input",
        help = "File to be read for input or \'-\' to use open stdin"
    )]
    file: String,
    #[structopt(
        long = "bytes",
        default_value = "1024",
        help = "The number of bytes to be read for each entropy calculation"
    )]
    bytes: u32,
    #[structopt(
        long = "csv",
        help = "Output as csv to stdout instead of using color-coding on the terminal.\nFormats as: <startbyte>;<entropy>"
    )]
    csv: bool,
}

fn main() {
    let cfg = Tropy::from_args();
    let mut r: Box<dyn BufRead> = {
        if cfg.file.eq("-") {
            let s = io::stdin();
            eprintln!("* Using stdin for input");
            let r = BufReader::with_capacity(2048usize, s);
            Box::new(r)
        } else {
            let s = match File::open(&cfg.file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Opening file failed with: {}", e.to_string());
                    exit(e.raw_os_error().unwrap_or(1))
                }
            };
            eprintln!("* Using {} for input", cfg.file);
            let r = BufReader::with_capacity(2048usize, s);
            Box::new(r)
        }
    };

    let chunksize = cfg.bytes as usize;
    let mut buf = vec![0u8; chunksize];
    let mut c = Calculator::new();
    let mut chunknum = 0usize;

    eprintln!("* Using chunks of {}bytes", chunksize);

    if !cfg.csv {
        // show colormap
        eprint!("* Color Scale: Entropy 0 "); // 25 chars

        let colormap_hue_start = 2.0 / 3.0;
        let colormap_hue_end = 1.0;
        let colormap_width = 80 - (25 + 2);
        let step = (colormap_hue_end - colormap_hue_start) / colormap_width as f64;

        (0..colormap_width)
            .map(|i| i as f64 * step + colormap_hue_start)
            .for_each(|hue| {
                eprint!(
                    "{}",
                    Rgb::from(Hsl {
                        h: hue * 360.0,
                        s: 1.0,
                        l: 0.5
                    })
                    .fg("█")
                )
            });

        eprint!(" 1"); // 2 chars
    } else {
        // use raw data
        eprintln!("Outputting raw data as csv in the format <startbyte>;<entropy/byte>");
        println!("\"start\";\"entropy\"");
    }

    while r
        .read_exact(&mut buf[..])
        .and_then(|_| c.write(&buf[..]))
        .is_ok()
    {
        let e = c.entropy();

        if !cfg.csv {
            if chunknum % 80 == 0 {
                println!();
            }
            // scale entropy to bits (i.e. value/8)
            // i.e. perfectly uniform data would have an entropy of 1 (i.e. 8bits/byte)
            let h = 240.0 + e / 8.0 * 120.0;
            print!("{}", Rgb::from(Hsl { h, s: 1.0, l: 0.5 }).fg("█"));
        } else {
            // output as csv
            println!("{};{:.6}", chunknum * chunksize, e);
        }

        chunknum += 1;
    }
}
