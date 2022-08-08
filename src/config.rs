use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

use spdx::expression::{ExprNode, Operator};
use spdx::{Expression, LicenseItem};

#[derive(Deserialize)]
pub(crate) struct Copyright {
    pub(crate) year: u16,
    pub(crate) holder: String,
}

#[derive(Deserialize)]
pub(crate) struct CustomLicense {
    pub(crate) name: String,
    pub(crate) content: String,
}

#[derive(Deserialize)]
pub(crate) enum LicenseData {
    /// fully custom license text as base64
    Custom(CustomLicense),
    /// Stock MIT license agreement with generated copyright field
    Mit(Vec<Copyright>),
}

impl LicenseData {
    pub(crate) fn name(&self) -> &str {
        match self {
            LicenseData::Custom(x) => &x.name,
            LicenseData::Mit(_) => "MIT",
        }
    }

    pub(crate) fn content(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            LicenseData::Custom(x) => {
                let data = base64::decode(&x.content)?;
                let decoded = String::from_utf8(data)?;
                Ok(decoded)
            }
            LicenseData::Mit(copyrights) => {
                let mut cursor = std::io::Cursor::new(Vec::new());
                for copyright in copyrights {
                    writeln!(cursor, "copyright {} {}", copyright.year, copyright.holder)?;
                }
                writeln!(cursor)?;
                write!(cursor, "{}", MIT_TEXT)?;
                Ok(String::from_utf8(cursor.into_inner())?)
            }
        }
    }
}

#[derive(Deserialize)]
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
    let config: Configuration = serde_json::from_reader(reader)?;
    Ok(config)
}

const MIT_TEXT: &str = r#"
Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files
(the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
"#;
