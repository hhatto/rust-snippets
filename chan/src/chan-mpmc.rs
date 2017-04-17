#[macro_use]
extern crate chan;

use std::thread;
use std::time::Duration;

fn producer(sdone: chan::Sender<i32>) {
    println!("start producer");

    for i in 0..10 {
        println!("run.send. i={:?}", i);
        sdone.send(i);
        thread::sleep(Duration::from_millis(100));
    }
    println!("end producer");
}

fn consumer(id: i32, rdone: chan::Receiver<i32>) {
    println!("start consumer {}", id);

    loop {
        chan_select! {
            rdone.recv() -> ret => {
                match ret {
                    None => break,
                    Some(v) => println!("id={}, recv. ret={:?}", id, v),
                }
            }
        }
    }

    println!("end consumer {}", id);
}

fn main() {
    let (sdone, rdone) = chan::sync(0);
    thread::spawn(move || producer(sdone));
    for id in 0..2 {
        let crdone = rdone.clone();
        thread::spawn(move || consumer(id, crdone));
    }

    let tick = chan::tick_ms(1000);
    let boom = chan::after_ms(5000);
    loop {
        chan_select! {
            default => { println!("."); thread::sleep(Duration::from_millis(1000)); },
            tick.recv() => println!("tick."),
            boom.recv() => break,
        }
    }
}
