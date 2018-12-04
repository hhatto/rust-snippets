use globset::GlobBuilder;

fn main() {
    let glob = GlobBuilder::new("bar.rs")
        .case_insensitive(true)
        .build().unwrap().compile_matcher();

    for name in vec!["bar.rs", "baR.rs", "foo/bar.rs", "Cargo.toml"] {
        if glob.is_match(name) {
            println!("{} is match", name);
        } else {
            println!("{} is not match", name);
        }
    }
}
