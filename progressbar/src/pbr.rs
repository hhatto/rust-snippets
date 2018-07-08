extern crate pbr;

use pbr::ProgressBar;
use std::thread;
use std::time::Duration;

fn simple() {
    const MAX: u64 = 10;
    let mut pb = ProgressBar::new(MAX);
    pb.format("╢▌▌░╟");
    for _ in 0..MAX {
        pb.inc();
        thread::sleep(Duration::from_millis(100));
    }
    pb.finish_print("done")
}

fn main() {
    simple();
}
