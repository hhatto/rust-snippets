use ssh2::Session;
use std::env;
use std::net::TcpStream;

pub fn ssh_login(tcp: &mut TcpStream, username: &'static str) -> Session {
    let mut session = Session::new().unwrap();
    session.handshake(&tcp).unwrap();
    let privatekey_filepath = env::home_dir().unwrap().join(".ssh/id_rsa");
    session.userauth_pubkey_file(username, None, privatekey_filepath.as_path(), None)
           .unwrap();

    assert!(session.authenticated());
    session
}
