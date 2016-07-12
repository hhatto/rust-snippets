extern crate unix_socket;

use std::env;
use std::io::prelude::*;
use unix_socket::UnixStream;

fn main() {
    let socket_path = env::args().skip(1).next().unwrap();
    let mut stream = UnixStream::connect(socket_path.as_str()).unwrap();
    stream.write_all(b"hello").unwrap();
}
