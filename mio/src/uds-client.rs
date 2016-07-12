extern crate mio;
extern crate bytes;

use std::env;
use std::path::Path;
use mio::*;
use mio::unix::UnixStream;

const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

struct TestHandler {
    client: UnixStream,
}

impl TestHandler {
    fn new(cli: UnixStream) -> TestHandler {
        TestHandler { client: cli }
    }

    fn handle_read(&mut self, _: &mut EventLoop<TestHandler>, token: Token, _: EventSet) {
        match token {
            SERVER => {}
            CLIENT => {}
            _ => panic!("unexpected token"),
        }
    }

    fn handle_write(&mut self, event_loop: &mut EventLoop<TestHandler>, _: Token, _: EventSet) {
        assert!(5 == self.client.try_write(&mut "hello".as_bytes()).unwrap().unwrap());
        event_loop.deregister(&self.client).unwrap();
        event_loop.timeout_ms(1, 200).unwrap();
    }
}

impl Handler for TestHandler {
    type Timeout = usize;
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<TestHandler>, token: Token, events: EventSet) {
        if events.is_readable() {
            self.handle_read(event_loop, token, events);
        }

        if events.is_writable() {
            self.handle_write(event_loop, token, events);
        }
    }

    fn timeout(&mut self, event_loop: &mut EventLoop<TestHandler>, _: usize) {
        event_loop.shutdown();
    }
}

fn main() {
    let socket_path = env::args().skip(1).next().unwrap();
    let mut event_loop = EventLoop::new().unwrap();
    let addr = Path::new(socket_path.as_str());
    let client = UnixStream::connect(&addr).unwrap();
    println!("connect");

    event_loop.register(&client, CLIENT, EventSet::writable(), PollOpt::level())
        .unwrap();

    let mut handler = TestHandler::new(client);
    event_loop.run(&mut handler).unwrap();
    println!("run.end");
}
