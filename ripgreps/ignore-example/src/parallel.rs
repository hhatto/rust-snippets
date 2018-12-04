use std::thread;
use ignore::{DirEntry, WalkBuilder};
use crossbeam_channel as channel;

fn main() {
    let (tx, rx) = channel::bounded::<DirEntry>(100);
    let mut builder = WalkBuilder::new("./");

    let stdout_thread = thread::spawn(move || {
        for dent in rx {
            println!("{}", dent.path().to_str().unwrap());
        }
    });

    builder.threads(8).build_parallel().run(|| {
        let tx = tx.clone();
        Box::new(move |result| {
            use ignore::WalkState::*;

            let _ = tx.send(result.unwrap());
            Continue
        })
    });

    drop(tx);
    stdout_thread.join().unwrap();
}
