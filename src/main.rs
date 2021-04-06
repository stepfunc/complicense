pub(crate) mod config;
pub(crate) mod github;
pub(crate) mod import;
pub(crate) mod options;

use crate::config::Configuration;
use crate::import::LicenseData;
use crate::options::Options;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();
    let config: Configuration = config::read_config_file(options.config_file)?;
    let entries = import::read_import_file(options.import_file)?;

    for entry in entries {
        if !options.ignore_list.contains(&entry.name) {
            println!("crate: {}", entry.name);
            let repo = entry
                .repository
                .clone()
                .unwrap_or_else(|| "none specified".to_owned());
            println!("repository: {}", repo);
            let license = entry
                .license
                .clone()
                .unwrap_or_else(|| "UNKNOWN".to_owned());
            println!("license: {}", license);
            println!("authors: {}", entry.authors);
            println!(
                "description: {}",
                entry
                    .description
                    .clone()
                    .unwrap_or_else(|| "none specified".to_owned())
            );

            let license: LicenseData = match config.get_license_data(&entry.name) {
                Some(data) => data.clone(),
                // if it's not in the configuration, grab the data from github API
                None => crate::github::get_license(&entry, &options.oauth_token)?,
            };

            println!();
            println!("{}", license.file_content);
        }
    }
    Ok(())
}
