extern crate protobuf;

mod person;

fn main() {
    let mut person = person::Person::new();

    person.name = "hoge".to_string();

    println!("p={:?}", person);
}
