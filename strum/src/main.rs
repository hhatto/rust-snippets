extern crate strum;
#[macro_use]
extern crate strum_macros;
use std::str::FromStr;
use std::string::ToString;

#[derive(ToString, EnumString, Debug)]
enum Browser {
    Chrome,
    Firefox,

    #[strum(disabled="true")]
    IE,

    #[strum(serialize="Microsoft Edge")]
    Edge
}

fn main() {
    let b = Browser::IE;
    println!("{:?}", b);
    let b = Browser::from_str("Chrome").unwrap();
    println!("{:?}", b);
    let b = match Browser::from_str("IE") {
        Ok(v) => v,
        Err(e) => { println!("disabled enum, error: {:?}", e); Browser::IE },
    };
    println!("{:?}", b);

    let b = Browser::Edge;
    println!("{}, {:?}", b.to_string(), b);
}
