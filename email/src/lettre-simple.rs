extern crate lettre;

use std::env::temp_dir;
use lettre::file::FileEmailTransport;
use lettre::{SimpleSendableEmail, EmailTransport, EmailAddress};

fn sendmail_with_file() {
    let t = temp_dir();
    println!("{:?}", t);
    let mut sender = FileEmailTransport::new(t);
    let email = SimpleSendableEmail::new(
        EmailAddress::new("from-address@localhost".to_string()),
        vec![EmailAddress::new("to-address@localhost".to_string())],
        "Message-ID".to_string(),
        "Hello rust mail body".to_string()
        );

    let result = sender.send(&email);
    println!("{}", result.is_ok());
}

fn main() {
    sendmail_with_file();
}
