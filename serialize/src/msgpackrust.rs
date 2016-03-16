extern crate rustc_serialize;
extern crate rmp_serialize as msgpack;

use rustc_serialize::{Encodable, Decodable};
use msgpack::{Encoder, Decoder};

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
struct Custom {
    id: u16,
    key: String,
}

fn main() {
    let val = (33u8, "Hello");
    let mut buf = [0u8; 13];

    let _ = val.encode(&mut Encoder::new(&mut &mut buf[..]));
    println!("encode: {:?}", buf);

    let mut decoder = Decoder::new(&buf[..]);
    let res: (u8, String) = Decodable::decode(&mut decoder).unwrap();
    println!("decode: {:?}", res);

    let custom_val = Custom {
        id: 9,
        key: "FooBar".to_string(),
    };
    let mut custom_buf = [0u8; 20];
    let _ = custom_val.encode(&mut Encoder::new(&mut &mut custom_buf[..]));
    println!("custom encode: {:?}", custom_buf);

    let mut custom_decoder = Decoder::new(&custom_buf[..]);
    let custom_res: Custom = Decodable::decode(&mut custom_decoder).ok().unwrap();
    println!("custom decode: {:?} {:?}", custom_res, custom_res.key);
}
