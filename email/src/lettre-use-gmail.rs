use std::env;
use std::path::Path;
use lettre::{SmtpClient, Transport};
use lettre::smtp::authentication::Credentials;
use lettre_email::EmailBuilder;

fn send_mail_via_gmail() {
    let host = "smtp.gmail.com";
    let username = env::var("GMAIL_USERNAME").unwrap();
    let password = env::var("GMAIL_PASSWORD").unwrap();
    let creds = Credentials::new(username.to_string(), password);
    let mut sender = SmtpClient::new_simple(host)
        .unwrap()
        .credentials(creds)
        .transport();

    let to_address = "hoge@example.com";
    let cc_address = "huga@example.com";
    let email = EmailBuilder::new()
        .to((to_address, "Hogeta Hogeo"))
        .from(username)
        .cc(cc_address)
        .subject("どうもxxです")
        .text("こんにちはRust")
        .attachment_from_file(Path::new("src/main.rs"), None, &mime::TEXT_PLAIN).expect("attach error")
        .build().expect("fail to build email");

    let result = sender.send(email.into());
    println!("{}", result.is_ok());
}

fn main() {
    send_mail_via_gmail();
}
