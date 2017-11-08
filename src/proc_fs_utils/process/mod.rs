mod parsers;
pub mod types;

use std::path::Path;
use std::io;
use std::fs::DirEntry;

use self::types::ProcessInfo;
use self::parsers::get_user_name;
use self::parsers::parse_login_uid;
use self::parsers::parse_command_line;

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

pub fn scan_process_entries() -> io::Result<Vec<ProcessInfo>> {
    let proc_path = Path::new("/proc");
    let mut processes: Vec<ProcessInfo> = vec![];

    for entry_result in proc_path.read_dir()? {
        let entry = entry_result?;

        if is_process_entry(&entry) {
            if let Ok(command_line) = parse_command_line(&entry.path()) {
                if !command_line.is_empty() {
                    if let Ok(uid) = parse_login_uid(&entry.path()) {
                        if let Ok(user_name) = get_user_name(&entry.path()) {
                            let process = ProcessInfo::new(&command_line, uid, &user_name);
                            processes.push(process);
                        }
                    }
                }
            }
        }
    }

    Ok(processes)
}

