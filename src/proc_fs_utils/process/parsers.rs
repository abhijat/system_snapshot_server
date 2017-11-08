use std::path::Path;
use std::ptr::null_mut;
use std::io::{Result, Read, Error, ErrorKind};
use std::ffi::CStr;

use libc;

// TODO use asref here
fn read_proc_file_content(path: &Path, file_name: &str) -> Result<String> {
    let command_line_path = path.join(file_name);

    if command_line_path.exists() {
        let mut f = ::std::fs::File::open(command_line_path)?;
        let mut buffer = String::new();

        f.read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        let error_message = format!("missing file: {:?}", command_line_path);
        let error = Error::new(ErrorKind::NotFound, error_message);
        Err(error)
    }
}

pub fn parse_command_line(path: &Path) -> Result<String> {
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

pub fn parse_login_uid(path: &Path) -> Result<u32> {
    let content = read_proc_file_content(path, "loginuid")?;

    match content.parse::<u32>() {
        Ok(uid) => Ok(uid),
        Err(e) => {
            let error = Error::new(ErrorKind::InvalidData, e);
            Err(error)
        }
    }
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

pub fn get_user_name(path: &Path) -> Result<String> {
    match parse_login_uid(path) {
        Ok(uid) => {
            unsafe {
                match get_user_name_unsafe(uid) {
                    Some(user_name) => Ok(user_name),
                    None => {
                        let msg = format!("no user name for id {}", uid);
                        let error = Error::new(ErrorKind::InvalidData, msg);
                        Err(error)
                    }
                }
            }
        }
        Err(e) => Err(e)
    }
}