use serde::Serialize;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedAsset {
    pub asset_id: String,
    pub name: String,
    pub md5ext: String,
    pub data_format: String,
}

impl SharedAsset {
    pub fn new(name: String) -> Self {
        Self {
            asset_id: String::new(),
            name,
            md5ext: String::new(),
            data_format: String::new(),
        }
    }

    pub fn resolve(&mut self) {
        self.asset_id =
            format!("{:x}", md5::compute(fs::read(&self.name).unwrap()));
        self.data_format = Path::new(&self.name)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap()
            .to_string();
        self.md5ext = format!("{}.{}", self.asset_id, self.data_format);
    }
}
