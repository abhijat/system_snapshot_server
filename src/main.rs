extern crate libc;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod proc_fs_utils;

use proc_fs_utils::process;

fn main() {
    let processes = process::scan_process_entries().unwrap();
    for process in &processes {
        let s = serde_json::to_string(&process).unwrap();
        println!("{}", s);
    }
}
