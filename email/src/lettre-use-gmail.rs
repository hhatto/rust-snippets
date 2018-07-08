extern crate mime;
extern crate lettre;
extern crate lettre_email;

use std::env;
use std::path::Path;
use lettre::smtp::SmtpTransport;
use lettre::EmailTransport;
use lettre::smtp::authentication::Credentials;
use lettre_email::EmailBuilder;

fn send_mail_via_gmail() {
    let host = "smtp.gmail.com";
    let username = env::var("GMAIL_USERNAME").unwrap();
    let password = env::var("GMAIL_PASSWORD").unwrap();
    let to_address = "hoge@example.com";
    let cc_address = "huga@example.com";
    let mut sender = SmtpTransport::simple_builder(host.to_string()).expect("builder error")
        .credentials(Credentials::new(username.clone(), password))
        .smtp_utf8(true)
        .build();
    let email = EmailBuilder::new()
        .to((to_address, "Hogeta Hogeo"))
        .from(username)
        .cc(cc_address)
        .subject("どうもxxです")
        .text("こんにちはRust")
        .attachment(Path::new("src/main.rs"), None, mime::TEXT_PLAIN).expect("attach error")
        .build().expect("h");

    let result = sender.send(&email);
    println!("{}", result.is_ok());
}

fn main() {
    send_mail_via_gmail();
}
