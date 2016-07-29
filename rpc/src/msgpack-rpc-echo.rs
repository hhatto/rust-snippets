extern crate msgpack_rpc;
extern crate rmp as msgpack;

use std::thread;
use msgpack::Value;
use msgpack_rpc::{Server, Dispatch, Client};

#[derive(Clone, Default)]
struct EchoServer;

impl Dispatch for EchoServer {
    fn dispatch(&mut self, method: &str, args: Vec<Value>) -> Result<Value, Value> {
        match method {
            "hello" => Ok(Value::Array(args.to_owned())),
            _ => Err(Value::String("Invalid method name.".to_owned())),
        }
    }
}

fn main() {
    let server = Server::bind("localhost:10009").unwrap();
    let mut client = Client::connect_socket(server.local_addr().unwrap());

    thread::spawn(move || {
        server.handle(EchoServer);
    });

    let ret = client.call("hello", vec![Value::String("Hello msgpack-rpc".to_owned())]);
    println!("{:?}", ret);
}
