use ignore::Walk;

fn main() {
    for result in Walk::new("./") {
        match result {
            Ok(entry) => println!("{}", entry.path().display()),
            Err(err) => println!("ERROR: {}", err),
        }
    }
}
