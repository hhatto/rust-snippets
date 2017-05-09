extern crate progress;

use std::thread;
use std::time::Duration;

fn main() {
    let mut bar = progress::Bar::new();
    bar.set_job_title("example");

    const MAX: i32 = 20;

    for _ in 0..MAX {
        thread::sleep(Duration::from_millis(100));
        bar.add_percent(100 / MAX);
    }

    bar.reach_percent(100);
    bar.jobs_done();
}
