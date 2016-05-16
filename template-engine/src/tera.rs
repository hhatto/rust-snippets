extern crate tera;
use std::env;
use std::path::Path;
use tera::{Tera, Context};

fn main() {
    let root_path = env::current_dir().unwrap();
    let template_dir = root_path.join(Path::new("templates/*"));
    let template_engine = Tera::new(template_dir.to_str().unwrap());
    let mut context = Context::new();

    let foo = "";
    let hello = "world";
    let vector = vec![1, 3, 6];
    context.add("foo", &foo);
    context.add("hello", &hello);
    context.add("vector", &vector);
    println!("{}", template_engine.render("hello.tera", context).unwrap());
}
