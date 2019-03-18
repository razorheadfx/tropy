extern crate colorful;
extern crate structopt;

use colorful::{Colorful, HSL};
use structopt::StructOpt;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use tropy::Calculator;

#[derive(Debug, StructOpt)]
struct Cfg {
    #[structopt(
        short = "f",
        long = "file",
        help = "File to be read for input. If no file is given, stdin is opened for reading"
    )]
    file: Option<String>,
    #[structopt(
        short = "n",
        long = "chunksize",
        default_value = "1024",
        help = "The number of bytes to be read for each entropy calculation"
    )]
    chunksize: u32,
    #[structopt(
        short = "c",
        long = "csv",
        help = "Output as csv to stdout instead of using color-coding on the terminal. Format: <startbyte>;<endbyte>;<entropy>"
    )]
    csv: bool,
}

fn display(e: f64, col: usize) {
    if col % 80 == 0 {
        println!();
    }
    // scale entropy to bits (i.e. value/8)
    // i.e. perfectly uniform data would have an entropy of 1 (i.e. 8bits/byte)
    let h = 240.0 / 360.0 + e / 8.0 * 120.0 / 360.0;
    print!("{}", "█".hsl(h as f32, 1.0, 0.5));
}

fn main() -> io::Result<()> {
    let cfg = Cfg::from_args();
    let mut r: Box<dyn BufRead> = match cfg.file {
        Some(f) => {
            let s = File::open(&f)?;
            eprintln!("* Using {} for input", f);
            let r = BufReader::with_capacity(2048usize, s);
            Box::new(r)
        }
        None => {
            let s = io::stdin();
            eprintln!("* Using stdin for input");
            let r = BufReader::with_capacity(2048usize, s);
            Box::new(r)
        }
    };

    let chunksize = cfg.chunksize as usize;
    let mut buf = vec![0u8; chunksize];
    let mut c = Calculator::new();
    let mut chunknum = 0usize;

    eprintln!("* Using chunks of {}bytes", chunksize);

    if !cfg.csv {
        // show colormap
        eprintln!(
            "* Color Scale: Entropy 0 {} 1",
            "█"
                .repeat(80 - 27)
                .gradient_with_color(HSL::new(2.0 / 3.0, 1.0, 0.5), HSL::new(1.0, 1.0, 0.5))
        );
    } else {
        // use raw data
        eprintln!("Outputting raw data as csv in the format <startbyte>;<endbyte>;<entropy>");
        println!("\"start\";\"end\";\"entropy\"");
    }

    while let Ok(_) = r.read_exact(&mut buf[..]).and_then(|_| c.write(&buf[..])) {
        // TODO: opt
        let e = c.entropy();

        if !cfg.csv {
            display(e, chunknum);
        } else {
            // output as csv
            println!(
                "{};{};{:.6}",
                chunknum * chunksize,
                (chunknum + 1) * chunksize,
                e
            );
        }

        chunknum += 1;
    }

    Ok(())
}
