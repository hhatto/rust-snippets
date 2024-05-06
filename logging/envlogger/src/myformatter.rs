#[macro_use]
extern crate log;
extern crate env_logger;

use std::io::Write;
use log::LevelFilter;
use env_logger::{Builder, Env};
use time::format_description::well_known::Rfc3339;

fn myformat() {
    let env = Env::new().filter("RUST_LOG");

    let mut builder = Builder::from_env(env);
    builder.format(|buf, record| {
        let ts = buf.timestamp();
        let style = buf.default_level_style(record.level());
        let file = record.file().unwrap();
        let line = record.line().unwrap();
        let now = time::OffsetDateTime::now_utc();
        let n = now.format(&Rfc3339).unwrap();
        writeln!(
            buf,
            "[{}] [{}:{}] {}({}) {}:{style}{}{style:#}",
            record.level(), file, line, n, ts, record.target(), record.args()
        )
    })
        // .filter(None, LevelFilter::Debug)
        .filter(None, LevelFilter::Info)
        .init();

    debug!("debug level msg");
    info!("info level msg");
    warn!("warn level msg");
    error!("error level msg");
}

fn main() {
    myformat();
}
