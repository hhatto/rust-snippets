#[macro_use]
extern crate dotenvy_macro;

const HOST: &str = dotenv!("HOST");

fn main() {
    println!("HOST: {}", HOST);
}
