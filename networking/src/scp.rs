extern crate networking;
use networking::sshutil;
use std::io::prelude::*;
use std::path::Path;

static USERNAME: &'static str = "username";
static REMOTE_ADDRESS: &'static str = "127.0.0.1:22";
static REMOTE_FILE: &'static str = "/path/to/remote";
static NEW_REMOTE_FILE: &'static str = "/path/to/remote.new";

fn main() {
    let (_tcp, session) = sshutil::ssh_login(REMOTE_ADDRESS, USERNAME);

    let (mut remote_file, stat) = session.scp_recv(Path::new(REMOTE_FILE))
                                         .unwrap();
    println!("remote file stat: {}", stat.size());

    let mut contents = Vec::new();
    remote_file.read_to_end(&mut contents).unwrap();

    let mut new_remote_file = session.scp_send(Path::new(NEW_REMOTE_FILE),
                                               0o644,
                                               stat.size(),
                                               None)
                                     .unwrap();
    new_remote_file.write_all(&contents).unwrap();
}
