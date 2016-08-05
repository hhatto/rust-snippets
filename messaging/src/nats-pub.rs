extern crate nats;
use nats::*;

fn main() {
    let mut nc = Client::new("nats://127.0.0.1:4222").unwrap();
    nc.set_synchronous(true);
    nc.set_name("rust.app");

    nc.publish("subject", "rust-nats message".as_bytes()).unwrap();
}
