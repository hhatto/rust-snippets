extern crate woothee;
use woothee::parse;

fn main() {
    let mut result = parse("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)");
    println!("{:?}", result);

    result = parse("Twitterbot/1.0");
    println!("{:?}", result);

    result = parse("Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0; Xbox)");
    println!("{:?}", result);
}
