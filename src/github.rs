use crate::import::LicenseData;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LicenseField {
    key: String,
    name: String,
    spdx_id: String,
}

#[derive(Debug, Deserialize)]
struct LicenseInfo {
    pub(crate) content: String,
    pub(crate) license: LicenseField,
}

#[derive(Debug)]
struct Coordinates {
    user: String,
    project: String,
}

impl Coordinates {
    fn license_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{}/{}/license",
            self.user, self.project
        )
    }
}

fn get_coordinates(entry: &crate::import::Entry) -> Result<Coordinates, &'static str> {
    let repo = match &entry.repository {
        Some(x) => x,
        None => return Err("no repository specified"),
    };

    match repo.strip_prefix("https://github.com/") {
        Some(suffix) => Ok(get_raw_coordinates(suffix)?),
        None => Err("not a Github repo"),
    }
}

fn get_raw_coordinates(suffix: &str) -> Result<Coordinates, &'static str> {
    let tokens: Vec<&str> = suffix.split('/').collect();

    if tokens.len() < 2 {
        return Err("insufficient tokens in github URL");
    }

    Ok(Coordinates {
        user: tokens[0].to_owned(),
        project: tokens[1].to_owned(),
    })
}

pub(crate) fn get_license(
    entry: &crate::import::Entry,
    oauth_token: &str,
) -> Result<LicenseData, Box<dyn std::error::Error>> {
    let coord = get_coordinates(entry)?;

    let client = reqwest::blocking::Client::new();

    let resp = client
        .get(coord.license_url())
        .header(ACCEPT, "Accept: application/vnd.github.v3+json")
        .header(USER_AGENT, "info@stepfunc.io")
        .header(AUTHORIZATION, format!("token {}", oauth_token))
        .send()?;

    let info = resp.json::<LicenseInfo>()?;

    let stripped: String = info
        .content
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let bytes = base64::decode(&stripped)?;
    let license_text = std::str::from_utf8(&bytes)?;

    Ok(LicenseData::new(info.license.name, license_text.to_owned()))
}
