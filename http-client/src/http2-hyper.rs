extern crate rustls;
extern crate futures;
extern crate h2;
extern crate hyper;
extern crate http;
extern crate tokio_core;
extern crate tokio_rustls;
extern crate webpki;
extern crate webpki_roots;

use std::net::ToSocketAddrs;
use futures::*;
use h2::client;
use http::{Method, Request};
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use rustls::Session;
use tokio_rustls::ClientConfigExt;
use webpki::DNSNameRef;

const ALPN_H2: &str = "h2";

static URL: &'static str = "http1://nghttp2.org/httpbin/get";

fn h2() {
    let tls_client_config = std::sync::Arc::new({
        let mut c = rustls::ClientConfig::new();
        c.root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
        c.alpn_protocols.push(ALPN_H2.to_owned());
        c
    });
    let dns_name = DNSNameRef::try_from_ascii_str("nghttp2.org").expect("fail dns");

    let mut core = Core::new().expect("fail new reactor gen");
    let handle = core.handle();
    let addr = "nghttp2.org:443".to_socket_addrs().expect("fail parse url").next().unwrap();

    let tcp = TcpStream::connect(&addr, &handle);

    let tcp = tcp.then(|res| {
        let tcp = res.expect("error1");
        tls_client_config.connect_async(dns_name, tcp)
            .then(|res| {
                let tls = res.expect("fail tls");
                {
                    let (_, session) = tls.get_ref();
                    let negotiated_protocol = session.get_alpn_protocol();
                    assert_eq!(Some(ALPN_H2), negotiated_protocol.as_ref().map(|x| &**x));
                }
                client::handshake(tls)
            })
            .then(|res| {
                let (mut client, h2) = res.expect("fail handshake");
                let req = Request::builder()
                    .method(Method::GET)
                    .uri(URL)
                    .body(())
                    .expect("fail build GET request");
                let (response, _) = client.send_request(req, true).expect("fail send request");
                let stream = response.and_then(|response| {
                    let (_, body) = response.into_parts();
                    body.for_each(|chunk| {
                        println!("RX: {:?}", chunk);
                        Ok(())
                    })
                });

                h2.join(stream)
            })
    });

    core.run(tcp).expect("fail core.run()");
}

fn main() {
    h2();
}

