use std::fs::File;
use std::io::Read;

fn get_cpu_info() -> Result<String, String> {
    let mut file = File::open("/proc/cpuinfo")
        .map_err(|e| e.to_string())?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .map_err(|e| e.to_string())?;

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_cpu_info() {
        let content = get_cpu_info();
        assert!(content.is_ok())
    }
}