#[macro_use]
extern crate serde_derive;

use std::io::Cursor;

use serde::{Deserialize, Serialize};
use rmp_serde::{Deserializer, Serializer};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct Custom {
    id: u16,
    key: String,
}

fn main() {
    let val = (11, "Hi");
    let mut buf = [0x00; 15];
    let _ = val.serialize(&mut Serializer::new(&mut &mut buf[..])).unwrap();
    println!("encode: {:?}", buf);

    let mut de = Deserializer::new(&buf[..]);
    let res: (u8, String) = Deserialize::deserialize(&mut de).unwrap();
    println!("decode: {:?}", res);

    let mut buf = Vec::new();
    let val = Custom {
        id: 33,
        key: "Hello".into(),
    };

    let _ = val.serialize(&mut Serializer::new(&mut buf)).unwrap();
    println!("encode: {:?}", buf);

    let cur = Cursor::new(&buf[..]);
    let mut de = Deserializer::new(cur);
    let res: (u16, String) = Deserialize::deserialize(&mut de).unwrap();
    println!("decode: {:?}", res);
}
