extern crate clap;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use clap::{App, Arg};

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
                if opt.skip_blank && l.is_empty() {
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
    let matches = App::new("cat")
        .version("v0.1")
        .arg(Arg::with_name("number")
            .short("n")
            .help("Number the output lines, starting at 1."))
        .arg(Arg::with_name("blank")
            .short("b")
            .help("Number the non-blank output lines, starting at 1."))
        .arg(Arg::with_name("files")
            .help("some files")
            .index(1)
            .multiple(true)
            .takes_value(true))
        .get_matches();
    let files = matches.values_of("files").unwrap();
    match matches.occurrences_of("number") {
        1 => opt.print_number = true,
        _ => {}
    }
    match matches.occurrences_of("blank") {
        1 => opt.skip_blank = true,
        _ => {}
    }
    for file in files {
        let path = Path::new(&file);
        print_lines(&path, &opt);
    }
}
