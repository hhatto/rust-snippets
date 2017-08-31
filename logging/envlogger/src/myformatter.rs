#[macro_use]
extern crate log;
extern crate env_logger;
extern crate time;

use std::env;
use log::{LogRecord, LogLevelFilter};
use env_logger::LogBuilder;


fn myformat() {
    let fmt = |record: &LogRecord| {
        let loc = record.location();
        let now = time::now();
        let n = time::strftime("%Y-%m-%d %H:%M:%S.%s", &now).expect("strftime() error");
        format!("[{}] [{}:{}] {} {}:{}  ({:?})",
                record.level(), loc.file(), loc.line(), n, record.target(), record.args(), record.metadata())
    };

    let mut builder = LogBuilder::new();
    builder.format(fmt).filter(None, LogLevelFilter::Info);

    if env::var("RUST_LOG").is_ok() {
        builder.parse(&env::var("RUST_LOG").unwrap());
    }

    builder.init().unwrap();

    error!("error level msg");
}

fn main() {
    myformat();
}
