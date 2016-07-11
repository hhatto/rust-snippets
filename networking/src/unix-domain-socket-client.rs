extern crate unix_socket;

use std::env;
use std::io::prelude::*;
use std::io::Cursor;
use unix_socket::UnixStream;

struct Header {
    ver: i8,
    psh: i8,
    len: i16,
    offset: i32,
    buf_len: i32,
}

impl Header {
    fn new() -> Header {
        Header {
            ver: 1,
            psh: 1,
            len: 12,
            offset: 0,
            buf_len: 0,
        }
    }

    fn bytes(&self) {}
}

fn main() {
    let mut socket_path = env::args().skip(1).next().unwrap();
    let mut stream = UnixStream::connect(socket_path.as_str()).unwrap();
    let h = Header::new();
    stream.write_all(b"hello").unwrap();
}
