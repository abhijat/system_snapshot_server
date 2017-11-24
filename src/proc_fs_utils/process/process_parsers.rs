use std::path::Path;
use std::ptr::null_mut;
use std::io::Read;
use std::ffi::CStr;
use std::fs::File;

use libc;

use failure::Error;

// TODO use asref here
fn read_proc_file_content(path: &Path, file_name: &str) -> Result<String, Error> {
    let command_line_path = path.join(file_name);

    if command_line_path.exists() {
        let mut f = File::open(command_line_path)?;

        let mut buffer = String::new();

        f.read_to_string(&mut buffer)?;

        Ok(buffer)
    } else {
        Err(format_err!("missing file: {:?}", command_line_path))
    }
}

fn get_row_with_key(path: &Path, file_name: &str, key: &str) -> Result<String, Error> {
    let content = read_proc_file_content(path, file_name)?;
    for line in content.lines() {
        if line.starts_with(key) {
            return Ok(line.to_owned());
        }
    }

    Err(format_err!("{} not found in {}", key, file_name))
}

pub fn parse_command_line(path: &Path) -> Result<String, Error> {
    let replace = |ch: u8| -> u8 {
        if ch == 0 { ' ' as u8 } else { ch }
    };

    read_proc_file_content(path, "cmdline").map(|s| {
        let nulls_removed = s.bytes()
            .map(replace)
            .collect::<Vec<u8>>();

        String::from_utf8(nulls_removed)
            .unwrap()
            .trim()
            .to_owned()
    })
}

pub fn parse_login_uid(path: &Path) -> Result<u32, Error> {
    let content = read_proc_file_content(path, "loginuid")?;
    let id = content.parse::<u32>()?;
    Ok(id)
}

unsafe fn get_user_name_unsafe(uid: u32) -> Option<String> {
    let mut passwd: libc::passwd = ::std::mem::zeroed();
    let mut buf = Vec::with_capacity(1024);

    let mut result = null_mut();
    let r = libc::getpwuid_r(
        uid,
        &mut passwd,
        buf.as_mut_ptr(),
        buf.capacity() as libc::size_t,
        &mut result
    );

    match r {
        0 if !result.is_null() => {
            let name_ptr = passwd.pw_name;
            let user_name = CStr::from_ptr(name_ptr).to_str().unwrap().to_owned();
            Some(user_name)
        }
        _ => None
    }
}

pub fn get_user_name(path: &Path) -> Result<String, Error> {
    match parse_login_uid(path) {
        Ok(uid) => {
            unsafe {
                match get_user_name_unsafe(uid) {
                    Some(user_name) => Ok(user_name),
                    None => Err(format_err!("no user name for id {}", uid))
                }
            }
        }
        Err(e) => Err(e)
    }
}

pub fn get_process_state(path: &Path) -> Result<String, Error> {
    let state_line = get_row_with_key(path, "status", "State:")?;
    let tokens: Vec<&str> = state_line.splitn(2, ":").collect();
    match tokens.len() {
        2 => Ok(tokens[1].trim().to_owned()),
        _ => Err(format_err!("invalid data: {}", state_line))
    }
}