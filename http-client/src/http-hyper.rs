extern crate hyper;
extern crate rustc_serialize;
use std::io::Read;
use hyper::Client;
use hyper::http::h1::Http11Protocol;
use hyper::net::DefaultConnector;

static URL: &'static str = "https://http2bin.org/get";

fn http11() {
    let connector = DefaultConnector::default();
    let client = Client::with_protocol(Http11Protocol::with_connector(connector));
    let mut res = client.get(URL).send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("{}", body);
}

fn main() {
    http11();
}
