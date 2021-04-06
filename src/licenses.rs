use crate::import::LicenseData;
use std::collections::HashMap;
use std::path::Path;

// lookup table for manually configured license information
pub(crate) struct LicenseTable {
    map: HashMap<String, LicenseData>,
}

impl LicenseTable {
    pub(crate) fn get(&self, name: &str) -> Option<&LicenseData> {
        self.map.get(name)
    }
}

fn get_license_table<P: AsRef<Path>>(path: P) -> Result<LicenseTable, Box<dyn std::error::Error>> {
    unimplemented!()
}
