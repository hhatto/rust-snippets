extern crate uap_rust;
use uap_rust::parser::Parser;

fn main() {
    let parser = Parser::new().unwrap();
    let mut result = parser.parse("Mozilla/4.0 (compatible; MSIE 8.0; Windows NT 6.1; \
                                   Trident/4.0)"
                                      .to_string());
    println!("{:?}", result);

    result = parser.parse("Twitterbot/1.0".to_string());
    println!("{:?}", result);

    result = parser.parse("Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; Trident/5.0; Xbox)"
                              .to_string());
    println!("{:?}", result);
}
