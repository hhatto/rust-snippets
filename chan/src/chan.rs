#[macro_use]
extern crate chan;
extern crate chan_signal;

use std::thread;
use std::time::Duration;
use chan_signal::Signal;

fn run(sdone: chan::Sender<()>) {
    println!("running");
    thread::sleep(Duration::from_millis(3000));

    println!("run.send");
    sdone.send(());
    println!("run.send.end");
}

fn main() {
    let signal = chan_signal::notify(&[
                                     Signal::INT,
                                     Signal::TERM,
    ]);
    let (sdone, rdone) = chan::sync(0);
    thread::spawn(move || run(sdone));

    let tick = chan::tick_ms(1000);
    let boom = chan::after_ms(5000);
    loop {
        chan_select! {
            default => { println!("."); thread::sleep(Duration::from_millis(1000)); },
            signal.recv() -> signal => {
                println!("signal.recv. sig={:?}", signal);
            },
            tick.recv() => println!("tick."),
            rdone.recv() => { println!("rdone"); return; },
            boom.recv() => { println!("boom!"); return; },
        }
    }
}
