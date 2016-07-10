extern crate websocket;
use std::thread;
use websocket::{Server, Message, Sender, Receiver};
use websocket::message::Type;
use websocket::header::WebSocketProtocol;
use websocket::result::WebSocketError;

fn main() {
    let server = Server::bind("127.0.0.1:9999").unwrap();

    for connection in server {
        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap();
            let headers = request.headers.clone();

            request.validate().unwrap();

            let mut response = request.accept();
            if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
                if protocols.contains(&("rust-websocket".to_string())) {
                    response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
                }
            }

            let mut client = response.send().unwrap();
            let ip = client.get_mut_sender()
                           .get_mut()
                           .peer_addr()
                           .unwrap();

            println!("Connection from {}", ip);

            let (mut sender, mut receiver) = client.split();

            let mut cnt: i32 = 0;
            for msg in receiver.incoming_messages() {
                cnt += 1;
                let msg: Message = match msg {
                    Ok(m) => m,
                    Err(WebSocketError::NoDataAvailable) => return, // connection close by client
                    Err(e) => {
                        println!("Receive error: {:?}", e);
                        return;
                    }
                };

                match msg.opcode {
                    Type::Close => {
                        let msg = Message::close();
                        sender.send_message(&msg).unwrap();
                        println!("Client {} disconnected", ip);
                        return;
                    }
                    Type::Ping => {
                        let msg = Message::pong(msg.payload);
                        sender.send_message(&msg).unwrap();
                    }
                    Type::Pong => {
                        println!("recv PONG");
                    }
                    _ => {
                        sender.send_message(&msg).unwrap();
                        println!("receive msg: echo: {:?}", msg);

                        if cnt % 2 == 1 {
                            let d = Vec::new();
                            sender.send_message(&Message::ping(d)).unwrap();
                            println!("send PING");
                        }
                    }
                }
            }
        });
    }
}
