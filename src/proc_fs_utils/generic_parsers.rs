use std::fs::File;
use std::io::Read;

use failure::Error;

use proc_fs_utils::types::CpuInfo;

pub fn get_cpu_info() -> Result<CpuInfo, Error> {
    let mut buffer = String::new();
    File::open("/proc/cpuinfo")?.read_to_string(&mut buffer)?;

    let mut num_cores = 0;
    let mut vendor: Option<String> = None;
    let mut model: Option<String> = None;

    for line in buffer.lines().filter(|line| line.contains(":")) {
        let tokens: Vec<String> = line.split(":")
            .map(|s: &str| { s.trim().to_owned() })
            .collect::<Vec<String>>();

        if tokens.len() != 2 {
            continue
        }

        let (key, value) = (tokens[0].as_str(), tokens[1].as_str());
        match key {
            "processor" => num_cores += 1,
            "vendor_id" if vendor.is_none() => vendor = Some(value.to_owned()),
            "model name" if model.is_none() => model = Some(value.to_owned()),
            _ => {}
        }
    }

    if num_cores > 0 && vendor.is_some() && model.is_some() {
        let cpu_info = CpuInfo::new(num_cores, vendor.unwrap().as_str(),
                                    model.unwrap().as_str());
        Ok(cpu_info)
    } else {
        Err(format_err!("unable to parse cpu information. not all required fields were found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cpu_info() {
        let result = get_cpu_info();
        assert!(result.is_ok());
        assert!(result.ok().unwrap().core_count > 0);
    }
}