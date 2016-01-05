extern crate yaml_rust;
use yaml_rust::YamlLoader;
use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn parse_yaml(yamlstr: &String) {
    let docs = YamlLoader::load_from_str(yamlstr).unwrap();
    let doc = &docs[0];

    let bad = &doc["domain"];
    if !bad.is_badvalue() {
        println!("{}", bad.as_str().unwrap());
    }

    let paths = &doc["paths"];
    if !paths.is_badvalue() {
        let pathvec = paths.as_vec().unwrap();
        for path in pathvec {
            for (key, value) in path.as_hash().unwrap() {
                println!("{:?} {}: {}",
                         path,
                         key.as_str().unwrap(),
                         value.as_str().unwrap());
            }
        }
    }
}

fn main() {
    if std::env::args().len() != 2 {
        println!("usage: PROG FILE.yaml");
        return;
    }
    let yamlfile = std::env::args().last().unwrap();
    let path = Path::new(&yamlfile);
    let display = path.display();

    let mut f = match File::open(&path) {
        Err(why) => panic!("could not open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Err(why) => panic!("could not read {}: {}", display, Error::description(&why)),
        Ok(_) => parse_yaml(&s),
    };
}
