extern crate url;
use url::{Url, UrlParser};

fn main() {
    let host = Url::parse("http://localhost:7777/").unwrap();
    let pairs = [("hoge", "fuga"), ("t", "zzz")];
    let mut target_url = UrlParser::new().base_url(&host).parse("/hello").unwrap();
    target_url.set_query_from_pairs(pairs.iter().map(|&(k, v)| (k, v)));

    println!("{}", target_url);
}
