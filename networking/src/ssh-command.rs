extern crate networking;
use networking::sshutil;
use std::net::TcpStream;
use std::io::prelude::*;

static USERNAME: &'static str = "username";
static REMOTE_ADDRESS: &'static str = "127.0.0.1:22";

fn main() {
    let mut tcp = TcpStream::connect(REMOTE_ADDRESS).unwrap();
    let session = sshutil::ssh_login(&mut tcp, USERNAME);

    let mut chan = session.channel_session().unwrap();
    chan.exec("ls").unwrap();
    let mut s = String::new();
    chan.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
