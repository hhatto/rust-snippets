#[macro_use]
extern crate log;
extern crate env_logger;

use log::Level;


fn level() {
    println!("{}, {}, {}, {}, {}",
             Level::Trace, Level::Debug, Level::Info, Level::Warn, Level::Error);
}

fn simple() {
    trace!("trace level msg");  // RUST_LOG=trace
    debug!("debug level msg");  // RUST_LOG=debug
    info!("info level msg");    // RUST_LOG=info
    warn!("warn level msg");    // RUST_LOG=warn
    error!("error level msg");  // RUST_LOG=error (defaul level)
}

fn mytarget() {
    debug!(target: "mytarget", "debug target msg"); // RUST_LOG=mytarget
    warn!(target: "mytarget", "warn target msg");   // RUST_LOG=mytarget=warn
}

fn main() {
    env_logger::init();

    level();
    simple();
    mytarget();

    //log::shutdown_logger().expect("shutdown logger error");
}
