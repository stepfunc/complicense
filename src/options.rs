use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "complicense", about = "OSS license report generator")]
pub(crate) struct Options {
    /// input JSON file from cargo-license
    #[structopt(short = "c", long = "config")]
    pub(crate) config_file: String,
    /// imported JSON file from cargo-license
    #[structopt(short = "i", long = "import")]
    pub(crate) import_file: String,
    #[structopt(short = "t", long = "token")]
    /// github access token
    pub(crate) oauth_token: String,
}
