extern crate libc;
extern crate iron;
extern crate serde;
extern crate serde_json;
extern crate fern;
extern crate chrono;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

mod proc_fs_utils;
mod http;

fn formatter(out: fern::FormatCallback, message: &std::fmt::Arguments, record: &log::LogRecord) {
    let timestamp = chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]");
    out.finish(format_args!(
        "{timestamp} [{target}] [{level}] {message}",
        timestamp = timestamp,
        target = record.target(),
        level = record.level(),
        message = message
    ));
}

fn setup_logging() {
    fern::Dispatch::new()
        .format(formatter)
        .level(log::LogLevelFilter::Info)
        .chain(std::io::stdout())
        .apply()
        .expect("failed to setup logging");
}

fn main() {
    setup_logging();
    http::start_server();
}
