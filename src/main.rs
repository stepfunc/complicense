pub(crate) mod config;
pub(crate) mod github;
pub(crate) mod import;
pub(crate) mod options;

use crate::options::Options;
use std::collections::HashSet;
use std::error::Error;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::from_args();

    let config = match config::read_config_file(options.config_file) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("error reading config file: {}", err);
            return Err(err);
        }
    };
    let entries = match import::read_import_file(options.import_file) {
        Ok(x) => x,
        Err(err) => {
            eprintln!("error reading dependencies file: {}", err);
            return Err(err);
        }
    };

    // some crates may be used twice with two different versions
    let mut processed: HashSet<String> = HashSet::new();

    for entry in entries.iter() {
        if config.ignore(&entry.name) {
            continue;
        }

        if !processed.insert(entry.name.clone()) {
            // already did this crate
            continue;
        }

        println!("crate: {}", entry.name);
        let repo = entry
            .repository
            .clone()
            .unwrap_or_else(|| "none specified".to_owned());
        println!("repository: {}", repo);
        println!(
            "authors: {}",
            entry
                .authors
                .clone()
                .unwrap_or_else(|| "not specified".to_string())
        );
        println!(
            "description: {}",
            entry
                .description
                .clone()
                .unwrap_or_else(|| "none specified".to_owned())
        );

        let crate_license = entry.license.clone().unwrap_or_else(|| "UNKNOWN".into());

        let license_content: String = match config.get_license_data(&entry.name) {
            Some(data) => {
                // use the license we look up
                println!("license: {}", data.license_name);
                data.file_content.clone()
            }
            // if it's not in the configuration, grab the data from github API
            None => {
                println!("license: {}", crate_license);
                config.verify_allowed(&crate_license)?;
                crate::github::get_license_text(entry, &options.oauth_token)?
            }
        };

        println!();
        println!("{}", license_content);
    }
    Ok(())
}
