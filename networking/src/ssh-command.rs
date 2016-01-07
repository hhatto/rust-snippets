extern crate networking;
use networking::sshutil;
use std::io::prelude::*;

static USERNAME: &'static str = "username";
static REMOTE_ADDRESS: &'static str = "127.0.0.1:22";

fn main() {
    let (_tcp, session) = sshutil::ssh_login(REMOTE_ADDRESS, USERNAME);

    let mut chan = session.channel_session().unwrap();
    chan.exec("ls").unwrap();
    let mut s = String::new();
    chan.read_to_string(&mut s).unwrap();
    println!("{}", s);
}
