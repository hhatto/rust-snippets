use ssh2::Session;
use std::net::TcpStream;

pub fn ssh_login(addr: &'static str, username: &'static str) -> (TcpStream, Session) {
    let tcp = TcpStream::connect(addr).unwrap();
    let mut session = Session::new().unwrap();
    session.handshake(&tcp).unwrap();
    let privatekey_filepath = dirs::home_dir().unwrap().join(".ssh/id_rsa");
    session.userauth_pubkey_file(username, None, privatekey_filepath.as_path(), None)
           .unwrap();

    assert!(session.authenticated());
    (tcp, session)
}
