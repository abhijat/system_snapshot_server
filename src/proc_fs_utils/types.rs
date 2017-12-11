use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct CpuInfo {
    pub core_count: u32,
    pub vendor: String,
    pub model_name: String,
}

impl CpuInfo {
    pub fn new(core_count: u32, vendor: &str, model_name: &str) -> Self {
        CpuInfo {
            core_count,
            vendor: vendor.to_owned(),
            model_name: model_name.to_owned()
        }
    }
}

impl fmt::Display for CpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "cpu count: {}, vendor: {}, model: {}",
            self.core_count,
            self.vendor,
            self.model_name
        )
    }
}