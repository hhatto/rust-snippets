extern crate websocket;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use websocket::{Message, Sender, Receiver};
use websocket::client::request::Url;
use websocket::client::Client;
use websocket::message::Type;
use websocket::stream::WebSocketStream;
use websocket::result::WebSocketError;
use websocket::sender as wsender;
use websocket::receiver as wreceiver;

fn sendloop(mut sender: wsender::Sender<WebSocketStream>, rx: mpsc::Receiver<Message>) {
    let mut cnt = 0;
    loop {
        cnt += 1;
        let text = format!("Hello {:?}", cnt).to_string();
        let msg = Message::text(text);
        let _ = match sender.send_message(&msg) {
            Ok(r) => r,
            Err(e) => {
                println!("Send error: {:?}", e);
                return;
            }
        };
        println!("send msg: {:?}", msg);

        let _ = match rx.try_recv() {
            Ok(m) => {
                match m.opcode {
                    Type::Pong => {
                        sender.send_message(&m);
                        println!("send PONG: {:?}", m);
                    }
                    _ => {
                        continue;
                    }
                }
            }
            Err(_) => {}
        };

        std::thread::sleep(Duration::from_millis(1500));
    }
}

fn recvloop(mut receiver: wreceiver::Receiver<WebSocketStream>, tx: mpsc::Sender<Message>) {
    for msg in receiver.incoming_messages() {
        let msg: Message = match msg {
            Ok(m) => m,
            Err(WebSocketError::NoDataAvailable) => return, // connection close by server
            Err(e) => {
                println!("Receive Loop: {:?}", e);
                break;
            }
        };

        match msg.opcode {
            Type::Close => {
                let _ = tx.send(Message::close());
                return;
            }
            Type::Ping => {
                match tx.send(Message::pong(msg.payload)) {
                    Ok(()) => println!("recv PING"),
                    Err(e) => {
                        println!("Receive Loop: {:?}", e);
                        return;
                    }
                }
            }
            _ => println!("recv msg: {:?}", msg),
        }
    }
}

fn main() {
    let url = Url::parse("ws://127.0.0.1:9999").unwrap();
    println!("Connect to {}", url);

    let req = Client::connect(url).unwrap();
    let res = req.send().unwrap();

    res.validate().unwrap();
    println!("connected");

    let (sender, receiver) = res.begin().split();
    let (tx, rx) = mpsc::channel();
    let send_loop = thread::spawn(move || sendloop(sender, rx));
    let receive_loop = thread::spawn(move || recvloop(receiver, tx));

    let _ = send_loop.join();
    let _ = receive_loop.join();
    println!("end");
}
