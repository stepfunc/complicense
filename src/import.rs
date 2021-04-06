use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize)]
pub(crate) struct Entry {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) authors: String,
    pub(crate) repository: Option<String>,
    pub(crate) description: Option<String>,
}

// the type we return
pub(crate) struct LicenseData {
    // name of the license
    pub(crate) name: String,
    // decoded license file content
    pub(crate) file_content: String,
}

impl LicenseData {
    pub(crate) fn new(name: String, file_content: String) -> Self {
        Self { name, file_content }
    }
}

pub(crate) fn read_import_file<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let entries: Vec<Entry> = serde_json::from_reader(reader)?;
    Ok(entries)
}
