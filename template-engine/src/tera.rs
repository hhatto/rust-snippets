use std::env;
use std::path::Path;
use tera::{Tera, Context};

fn main() {
    let root_path = env::current_dir().unwrap();
    let template_dir = root_path.join(Path::new("templates/*.tera"));
    let template_engine = match Tera::new(template_dir.to_str().unwrap()) {
        Ok(t) => t,
        Err(e) => panic!("tera::new error: {}", e),
    };
    let mut context = Context::new();

    let foo = "";
    let hello = "world";
    let vector = vec![1, 3, 6];
    context.insert("foo", &foo);
    context.insert("hello", &hello);
    context.insert("vector", &vector);
    println!("{}", template_engine.render("hello.tera", &context).unwrap());
}
