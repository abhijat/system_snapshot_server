use std::fs::File;
use std::io::Read;

use failure::Error;

fn get_cpu_info() -> Result<String, Error> {
    let mut buffer = String::new();
    File::open("/proc/cpuinfo")?.read_to_string(&mut buffer)?;
    Ok(buffer)
}
