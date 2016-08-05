extern crate woothee;
use woothee::parser::Parser;

fn main() {
    let parser = Parser::new();
    let mut result = parser.parse("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; Trident/4.0)");
    println!("{:?}", result);

    result = parser.parse("Twitterbot/1.0");
    println!("{:?}", result);

    result = parser.parse("Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0; Xbox)");
    println!("{:?}", result);
}
