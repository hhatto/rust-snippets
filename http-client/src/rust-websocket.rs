extern crate websocket;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::net::TcpStream;
use websocket::{Message, OwnedMessage};
use websocket::client::ClientBuilder;
use websocket::result::WebSocketError;
use websocket::sync::sender as wsender;
use websocket::sync::receiver as wreceiver;

fn sendloop(mut sender: wsender::Writer<TcpStream>, rx: mpsc::Receiver<OwnedMessage>) {
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

        let message = match rx.try_recv() {
            Ok(m) => m,
            Err(_) => {
                return;
            }
        };
        match message {
            OwnedMessage::Close(_) => {
                return;
            }
            _ => (),
        }

        match sender.send_message(&message) {
            Ok(()) => (),
            Err(e) => {
                println!("Send Loop: {:?}", e);
                let _ = sender.send_message(&Message::close());
                return;
            }
        }
        std::thread::sleep(Duration::from_millis(1500));
    }
}

fn recvloop(mut receiver: wreceiver::Reader<TcpStream>, tx: mpsc::Sender<OwnedMessage>) {
    for msg in receiver.incoming_messages() {
        let msg: OwnedMessage = match msg {
            Ok(m) => m,
            Err(WebSocketError::NoDataAvailable) => return, // connection close by server
            Err(e) => {
                println!("Receive Loop First: {}", e);
                break;
            }
        };

        match msg {
            OwnedMessage::Close(_) => {
                println!("recv Close");
                let _ = tx.send(OwnedMessage::Close(None));
                return;
            }
            OwnedMessage::Ping(ping) => {
                println!("recv Ping");
                match tx.send(OwnedMessage::Pong(ping)) {
                    Ok(()) => println!("send Pong"),
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
    let client = ClientBuilder::new("ws://127.0.0.1:9999")
        .expect("fail new ws client")
        .add_protocol("rust-websocket")
        .connect_insecure().unwrap();
    println!("connected");

    let (receiver, sender) = client.split().expect("fail client split()");
    let (tx, rx) = mpsc::channel();
    let send_loop = thread::spawn(move || sendloop(sender, rx));
    let receive_loop = thread::spawn(move || recvloop(receiver, tx));

    let _ = send_loop.join();
    let _ = receive_loop.join();
    println!("end");
}
