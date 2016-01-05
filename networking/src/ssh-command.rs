extern crate ssh2;
use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
use std::env;

static USERNAME: &'static str = "username";
static REMOTE_ADDRESS: &'static str = "127.0.0.1:22";

fn main() {
    let tcp = TcpStream::connect(REMOTE_ADDRESS).unwrap();
    let mut session = Session::new().unwrap();
    session.handshake(&tcp).unwrap();
    let privatekey_filepath = env::home_dir().unwrap().join(".ssh/id_rsa");
    session.userauth_pubkey_file(USERNAME, None, privatekey_filepath.as_path(), None).unwrap();

    assert!(session.authenticated());

    let mut chan = session.channel_session().unwrap();
    chan.exec("ls").unwrap();
    let mut s = String::new();
    chan.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
