extern crate solicit;
use solicit::http::client::CleartextConnector;
use solicit::client::SimpleClient;
use std::str;

static HOST: &'static str = "http2bin.org";

fn main() {
    let connector = CleartextConnector::new(HOST);
    let mut client = SimpleClient::with_connector(connector).unwrap();
    let resp = client.get(b"/get", &[]).unwrap();

    println!("{}", str::from_utf8(&resp.body).unwrap());
}
