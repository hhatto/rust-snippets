extern crate ws;
extern crate env_logger;

use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};

const WS_URL: &str = "ws://echo.websocket.org";
const WSS_URL: &str = "wss://echo.websocket.org";

struct Client {
    out: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out.send("Hello ws-rs (wss)")
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("recv msg: {}", msg);
        self.out.close(CloseCode::Normal)
    }
}

fn main() {
    env_logger::init();

    connect(WS_URL, |out| {
        out.send("Hello ws-rs").unwrap();

        move |msg| {
            println!("msg recv: {}", msg);
            out.close(CloseCode::Normal)
        }
    }).expect("fail wss connect");

    connect(WSS_URL, |out| Client { out: out } ).unwrap()
}
