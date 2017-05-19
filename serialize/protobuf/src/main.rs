extern crate protobuf;

mod person;

fn main() {
    let mut person = person::Person::new();

    person.set_name("hoge".to_string());

    println!("p={:?}", person);
}
