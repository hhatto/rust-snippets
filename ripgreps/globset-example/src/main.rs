use globset::Glob;

fn main() {
    let glob = Glob::new("*.rs").unwrap().compile_matcher();

    for name in vec!["foo.rs", "foo/bar.rs", "Cargo.toml"] {
        if glob.is_match(name) {
            println!("{} is match", name);
        } else {
            println!("{} is not match", name);
        }
    }
}
