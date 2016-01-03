extern crate getopts;
use getopts::Options;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;

struct MyOptions {
    print_number: bool,
    skip_blank: bool,
}

fn print_lines(path: &Path, opt: &MyOptions) {
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
                if opt.skip_blank && l == "".to_string() {
                    println!("");
                    continue;
                }

                if opt.print_number || opt.skip_blank {
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
    let mut opt = MyOptions {
        print_number: false,
        skip_blank: false,
    };
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("n", "", "Number the output lines, starting at 1.");
    opts.optflag("b", "", "Number the non-blank output lines, starting at 1.");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(_) => {
            print!("{}", opts.usage("Usage: cat [options] [FILE]"));
            return;
        }
    };

    opt.print_number = matches.opt_present("n");
    opt.skip_blank = matches.opt_present("b");

    let files = matches.free;

    for arg in files {
        let path = Path::new(&arg);

        print_lines(&path, &opt);
    }
}
