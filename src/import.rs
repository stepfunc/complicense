use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Deserialize)]
pub(crate) struct Entry {
    pub(crate) name: String,
    pub(crate) authors: Option<String>,
    pub(crate) license: Option<String>,
    pub(crate) repository: Option<String>,
    pub(crate) description: Option<String>,
}

// the type we return
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct LicenseData {
    // name of the license
    pub(crate) license_name: String,
    // decoded license file content
    pub(crate) file_content: String,
}

pub(crate) fn read_import_file<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<Entry>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let entries: Vec<Entry> = serde_json::from_reader(reader)?;
    Ok(entries)
}
