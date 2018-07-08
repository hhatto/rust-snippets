extern crate url;
use url::Url;

fn main() {
    let url = Url::parse("http://localhost:7777/").expect("fail Url::parse()");
    let mut target_url = url.join("/hello").expect("fail url join()");
    target_url.set_query(Some("hoge=fuga"));
    target_url.set_query(Some("t=zzz"));

    println!("{}", target_url);
}
