use globset::{Glob, GlobSetBuilder};

fn main() {
    let mut builder = GlobSetBuilder::new();
    builder.add(Glob::new("foo/{foo,bar}.rs").unwrap());
    builder.add(Glob::new("Cargo.*").unwrap());

    let globs = builder.build().unwrap();
    for name in vec!["foo/foo.rs", "foo/bar.rs", "foo/baz.rs", "Cargo.toml", "Cargo.lock"] {
        if globs.is_match(name) {
            println!("{} is match", name);
        } else {
            println!("{} is not match", name);
        }
    }
}
