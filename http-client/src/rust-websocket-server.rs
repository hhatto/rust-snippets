extern crate websocket;
use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;
use websocket::result::WebSocketError;

fn main() {
    let server = Server::bind("127.0.0.1:9999").unwrap();

    for request in server.filter_map(Result::ok) {
        thread::spawn(move || {
            if !request.protocols().contains(&("rust-websocket".to_string())) {
                request.reject().expect("fail request reject()");
                return;
            }

            let client = request.accept().expect("fail accept");
            let ip = client.peer_addr().expect("fail peer_addr()");
            println!("Connection from {}", ip);

            let (mut receiver, mut sender) = client.split().expect("fail client.split()");

            let mut cnt: i32 = 0;
            for msg in receiver.incoming_messages() {
                cnt += 1;
                let msg: OwnedMessage = match msg {
                    Ok(m) => m,
                    Err(WebSocketError::NoDataAvailable) => return, // connection close by client
                    Err(e) => {
                        println!("Receive error: {:?}", e);
                        return;
                    }
                };

                match msg {
                    OwnedMessage::Close(_) => {
                        let msg = OwnedMessage::Close(None);
                        sender.send_message(&msg).unwrap();
                        println!("Client {} disconnected", ip);
                        return;
                    }
                    OwnedMessage::Ping(ping) => {
                        let msg = OwnedMessage::Pong(ping);
                        sender.send_message(&msg).unwrap();
                    }
                    OwnedMessage::Pong(_) => {
                        println!("recv PONG");
                    }
                    _ => {
                        sender.send_message(&msg).unwrap();
                        println!("receive msg: echo: {:?}", msg);

                        if cnt % 2 == 1 {
                            let d = Vec::new();
                            sender.send_message(&OwnedMessage::Ping(d)).unwrap();
                            println!("send PING");
                        }
                    }
                }
            }
        });
    }
}
