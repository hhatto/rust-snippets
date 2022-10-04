use std::error::Error;
use std::net::ToSocketAddrs;
use h2::client;
use http::{HeaderMap, Request};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use tokio_rustls::rustls::{OwnedTrustAnchor, RootCertStore, ServerName};

const ALPN_H2: &str = "h2";
static URL: &'static str = "https://http2.github.io";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let tls_client_config = std::sync::Arc::new({
        let mut root_store = RootCertStore::empty();
        root_store.add_server_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
            OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject,
                ta.spki,
                ta.name_constraints,
            )
        }));

        let mut c = tokio_rustls::rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        c.alpn_protocols.push(ALPN_H2.as_bytes().to_owned());
        c
    });

     let addr = "http2.github.io:443"
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();

    println!("address: {:?}", addr);

    let tcp = TcpStream::connect(&addr).await?;
    let dns_name = ServerName::try_from("http2.github.io").unwrap();
    let connector = TlsConnector::from(tls_client_config);
    let res = connector.connect(dns_name, tcp).await;
    let tls = res.unwrap();
    {
        let (_, session) = tls.get_ref();
        let negotiated_protocol = session.alpn_protocol();
        assert_eq!(
            Some(ALPN_H2.as_bytes()),
            negotiated_protocol.as_ref().map(|x| &**x)
        );
    }

    let (mut client, h2) = client::handshake(tls).await?;

    let request = Request::builder()
        .uri(URL)
        .body(())
        .unwrap();

    println!("request: {:?}", request);

    let mut trailers = HeaderMap::new();
    trailers.insert("zomg", "hello".parse().unwrap());

    let (response, mut stream) = client.send_request(request, false).unwrap();

    stream.send_trailers(trailers).unwrap();

    tokio::spawn(async move {
        if let Err(e) = h2.await {
            println!("err={:?}", e);
        }
    });

    let response = response.await?;
    println!("response: {:?}", response);

    {
        let headers = response.headers();
        for header in headers.iter() {
            println!("headers: {:?}", header);
        }
    }
    let mut body = response.into_body();

    while let Some(chunk) = body.data().await {
        println!("body: {:?}", chunk?);
    }

    Ok(())
}
