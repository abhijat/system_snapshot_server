use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct ProcessInfo {
    pub command_line: String,
    pub uid: u32,
    pub user_name: String,
}

impl ProcessInfo {
    pub fn new(_command_line: &str, uid: u32, _user_name: &str) -> Self {
        ProcessInfo {
            command_line: _command_line.to_owned(),
            uid,
            user_name: _user_name.to_owned()
        }
    }
}

impl fmt::Display for ProcessInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "command: {}, uid: {}, username: {}",
            self.command_line,
            self.uid,
            self.user_name
        )
    }
}