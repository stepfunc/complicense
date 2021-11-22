use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::import::LicenseData;
use spdx::expression::{ExprNode, Operator};
use spdx::{Expression, LicenseItem};

#[derive(Debug, Deserialize)]
pub(crate) struct Configuration {
    /// list of crates to ignore
    ignore: Vec<String>,
    /// whitelist of allowed licenses
    allowed_licenses: Vec<String>,
    /// lookup table for manually configured license information
    crates: HashMap<String, LicenseData>,
}

impl Configuration {
    pub(crate) fn get_license_data(&self, crate_name: &str) -> Option<&LicenseData> {
        self.crates.get(crate_name)
    }

    pub(crate) fn verify_allowed(&self, license: &str) -> Result<(), Box<dyn std::error::Error>> {
        let expr = match Expression::parse(license) {
            Ok(expr) => expr,
            Err(err) => return Err(format!("SPDX parse error: {}", err).into()),
        };

        let licenses: Result<Vec<String>, _> = expr
            .iter()
            .flat_map(|x| match x {
                ExprNode::Op(op) => match op {
                    Operator::And => Some(Err("SPDX AND operator not allowed".to_string())),
                    Operator::Or => None,
                },
                ExprNode::Req(lic) => match &lic.req.license {
                    LicenseItem::Spdx { id, or_later: _ } => Some(Ok(id.name.to_string())),
                    LicenseItem::Other { .. } => {
                        Some(Err("SDPX other license types not allowed".to_string()))
                    }
                },
            })
            .collect();

        let licenses = licenses?;

        if !licenses.iter().any(|x| self.allowed_licenses.contains(x)) {
            return Err(format!("Could not find allowed license for: {}", license).into());
        }

        Ok(())
    }

    pub(crate) fn ignore(&self, name: &str) -> bool {
        self.ignore.contains(&name.to_owned())
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
