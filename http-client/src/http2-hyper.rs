extern crate hyper;
use std::io;
use std::io::Read;
use hyper::Client;
use hyper::http::h2;
use hyper::http::h2::Http2Protocol;
use hyper::net::{HttpsConnector, Openssl};

static URL: &'static str = "https://http2bin.org/get";
static CERT_FILE: &'static str = "tmp.crt";
static KEY_FILE: &'static str = "tmp.key";

// http -> OK
// https -> Error:
//          thread '<main>' panicked at 'called `Result::unwrap()` on an `Err` value:
//          Io(Error { repr: Custom(Custom { kind: InvalidInput, error: StringError("Invalid scheme for
//          Http") }) })', ../src/libcore/result.rs:738
fn http2_with_error() {
    let client = Client::with_protocol(h2::new_protocol());
    let mut res = client.get(URL).send().unwrap();

    println!("Response: {}", res.status);
    println!("Headers:\n{}", res.headers);
    io::copy(&mut res, &mut io::stdout()).unwrap();
}

fn http2() {
    let ssl = Openssl::with_cert_and_key(CERT_FILE, KEY_FILE).unwrap();
    let ssl_connector = HttpsConnector::new(ssl);
    let http2_client = Client::with_protocol(Http2Protocol::with_connector(ssl_connector));
    let mut res = http2_client.get(URL).send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("{}", body);
}

fn main() {
    http2();
    http2_with_error();
}

