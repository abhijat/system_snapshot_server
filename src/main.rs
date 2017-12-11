#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

extern crate libc;
extern crate iron;
extern crate serde;
extern crate serde_json;
extern crate chrono;

mod proc_fs_utils;
mod http;

fn setup_logging() {
    ::pretty_env_logger::init().unwrap();
}

fn main() {
    setup_logging();
    http::start_server();
}
