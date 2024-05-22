use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().expect(".env file not found");

    for (key, value) in env::vars() {
        println!("{key}: {value}");
    }
}
