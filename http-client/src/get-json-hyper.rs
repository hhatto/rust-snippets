extern crate hyper;
extern crate rustc_serialize;
use std::io::Read;
use std::collections::HashMap;

use rustc_serialize::json::Json;
use hyper::Client;
use hyper::header::Connection;

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct QiitaResp {
    id: HashMap<String, String>
}

static URL: &'static str = "http://qiita.com/api/v2/items?per_page=2";

pub fn parse_json(jsonstr: &String) {
    let data = Json::from_str(&jsonstr).unwrap();
    let items = data.as_array().unwrap();

    for d in items.iter() {
        let item = d.as_object().unwrap().get("title").unwrap();
        let datetime = d.as_object().unwrap().get("updated_at").unwrap();
        println!("{} {}", match *datetime {
            Json::String(ref v) => format!("{}", v),
            _ => format!(""),
        },
        match *item {
            Json::String(ref v) => format!("{}", v),
            _ => format!(""),
        });
    }
}

fn main() {
    let client = Client::new();
    let mut res = client.get(URL)
        .header(Connection::close())
        .send().unwrap();

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    //println!("Resp: {}", body);
    parse_json(&body);
}
