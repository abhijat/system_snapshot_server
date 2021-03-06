mod process_parsers;
mod parsers;
pub mod types;

use std::path::Path;
use std::fs::DirEntry;

use failure::{err_msg, Error};

use self::types::ProcessInfo;
use self::process_parsers::get_user_name;
use self::process_parsers::parse_login_uid;
use self::process_parsers::parse_command_line;
use self::process_parsers::get_process_state;

fn is_process_entry(entry: &DirEntry) -> bool {
    match entry.path().file_name() {
        Some(filename) => {
            match filename.to_str() {
                Some(value) => value.parse::<i32>().is_ok(),
                None => false
            }
        }
        None => false
    }
}

fn scan_process_entry(path: &Path) -> Result<ProcessInfo, Error> {
    let command_line = parse_command_line(path)?;
    if command_line.is_empty() {
        return Err(err_msg("no command line"));
    }

    let uid = parse_login_uid(path)?;
    let user_name = get_user_name(path)?;
    let state = get_process_state(path)?;
    Ok(ProcessInfo::new(&command_line, uid, &user_name, &state))
}

pub fn scan_process_entries() -> Result<Vec<ProcessInfo>, Error> {
    let proc_path = Path::new("/proc");
    let mut processes: Vec<ProcessInfo> = vec![];

    for entry_result in proc_path.read_dir()? {
        let entry = entry_result?;

        if is_process_entry(&entry) {
            if let Ok(process) = scan_process_entry(&entry.path()) {
                processes.push(process);
            }
        }
    }

    Ok(processes)
}

