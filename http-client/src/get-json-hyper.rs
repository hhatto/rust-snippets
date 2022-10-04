use std::io::Read;
use rustc_serialize::json::Json;
use hyper::Client;
use hyper::body::Buf;
use hyper_tls::HttpsConnector;

static REQUEST_URL: &'static str = "https://qiita.com/api/v2/items?per_page=2";

pub fn parse_json(jsonstr: &String) {
    let data = Json::from_str(&jsonstr).unwrap();
    let items = data.as_array().unwrap();

    for d in items.iter() {
        let item = d.as_object().unwrap().get("title").unwrap();
        let datetime = d.as_object().unwrap().get("updated_at").unwrap();
        println!("{} {}",
                 match *datetime {
                     Json::String(ref v) => format!("{}", v),
                     _ => format!(""),
                 },
                 match *item {
                     Json::String(ref v) => format!("{}", v),
                     _ => format!(""),
                 });
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let url = REQUEST_URL.parse()?;

    let res = client.get(url).await?;
    let body = hyper::body::aggregate(res).await?;

    let mut bbody = String::new();
    match body.reader().read_to_string(&mut bbody) {
        Ok(_) => parse_json(&bbody),
        Err(e) => println!("error: {}", e),
    };

    Ok(())
}
