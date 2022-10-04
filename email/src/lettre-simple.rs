use lettre::{SmtpClient, Transport};
use lettre_email::EmailBuilder;

fn sendmail_with_file() {
    let mut sender = SmtpClient::new_unencrypted_localhost().unwrap().transport();
    let email = EmailBuilder::new()
        .from("from-address@localhost")
        .to("to-address@localhost")
        .subject("Message-ID")
        .body("Hello rust mail body".to_string())
        .build().expect("fail to build email");

    let result = sender.send(email.into());
    println!("{}", result.is_ok());
}

fn main() {
    sendmail_with_file();
}
