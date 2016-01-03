extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, List};
use std::io::BufReader;
use std::io::prelude::*;
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
    let mut files: Vec<String> = vec![];
    let mut opt = MyOptions {
        print_number: false,
        skip_blank: false,
    };

    {
        let mut parser = ArgumentParser::new();
        parser.set_description("cat");
        parser.refer(&mut opt.print_number)
              .add_option(&["-n"], StoreTrue, "Number the output lines, starting at 1.");
        parser.refer(&mut opt.skip_blank)
              .add_option(&["-b"], StoreTrue, "Number the non-blank output lines, starting at 1.");
        parser.refer(&mut files)
              .add_argument("file", List, "some files");
        parser.parse_args_or_exit();
    }

    for file in files {
        let path = Path::new(&file);
        print_lines(&path, &opt);
    }
}
