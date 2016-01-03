use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn main() {
    for arg in std::env::args().skip(1) {
        let path = Path::new(&arg);
        let display = path.display();

        let mut f = match File::open(&path) {
            Err(why) => panic!("could not open {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };

        let mut s = String::new() ;
        match f.read_to_string(&mut s) {
            Err(why) => panic!("could not read {}: {}", display, Error::description(&why)),
            Ok(_) => print!("=== {} ===\n{}", display, s),
        };
    }
}
