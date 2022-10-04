use std::io::Read;
use hyper::Client;
use hyper::body::Buf;
use hyper_tls::HttpsConnector;

static URL: &'static str = "https://httpbin.org/get";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let res = client.get(URL.parse()?).await?;

    let body = hyper::body::aggregate(res).await?;
    let mut bbody = String::new();

    match body.reader().read_to_string(&mut bbody) {
        Ok(_) => println!("{}", bbody),
        Err(e) => println!("error: {}", e),
    };

    Ok(())
}
