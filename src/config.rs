use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::import::LicenseData;

// lookup table for manually configured license information
#[derive(Debug, Deserialize)]
pub(crate) struct Configuration {
    crates: HashMap<String, LicenseData>,
}

impl Configuration {
    pub(crate) fn get_license_data(&self, crate_name: &str) -> Option<&LicenseData> {
        self.crates.get(crate_name)
    }
}

pub(crate) fn read_config_file<P: AsRef<Path>>(
    path: P,
) -> Result<Configuration, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut config: Configuration = serde_json::from_reader(reader)?;
    // now we need to process the base64 encoded license content
    for (_, data) in config.crates.iter_mut() {
        let bytes = base64::decode(&data.file_content)?;
        data.file_content = std::str::from_utf8(&bytes)?.to_owned();
    }
    Ok(config)
}
