pub(crate) mod github;
pub(crate) mod import;
pub(crate) mod licenses;
pub(crate) mod options;

use std::fs::File;
use std::io::BufReader;
use structopt::StructOpt;

use crate::import::Entry;
use crate::options::Options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();
    let entries = import::read_import_file(options.input_file)?;

    for entry in entries {
        if !options.ignore_list.contains(&entry.name) {
            println!("crate: {}", entry.name);
            println!("repository: {:?}", entry.repository);
            let license = crate::github::get_license(&entry, &options.oauth_token)?;
            println!("license: {}", license.name);
            println!();
            println!("{}", license.file_content);
        }
    }
    Ok(())
}
