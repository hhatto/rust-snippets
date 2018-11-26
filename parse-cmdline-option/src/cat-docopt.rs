#[macro_use]
extern crate serde;
extern crate docopt;
use docopt::Docopt;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::Path;

static USAGE: &'static str = "
cat

Usage:
  cat-docopt [-n] [-b] [<file>...]

Options:
  -b    Number the non-blank output lines, starting at 1.
  -n    Number the output lines, starting at 1.
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_b: bool,
    flag_n: bool,
    arg_file: Vec<String>,
}

fn print_lines(path: &Path, opt: &Args) {
    let display = path.display();
    let f = match File::open(&path) {
        Err(why) => panic!("could not open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let buf = BufReader::new(&f);
    let mut n = 1;
    for line in buf.lines() {
        match line {
            Ok(l) => {
                if opt.flag_b && l == "".to_string() {
                    println!("");
                    continue;
                }

                if opt.flag_n || opt.flag_b {
                    println!(" {:5}  {}", n, l);
                } else {
                    println!("{}", l);
                }
            }
            Err(_) => {}
        };
        n += 1;
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.deserialize())
                         .unwrap_or_else(|e| e.exit());

    for arg in &args.arg_file {
        let path = Path::new(&arg);

        print_lines(&path, &args);
    }
}
