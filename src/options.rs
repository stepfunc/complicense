use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "complicense", about = "OSS license report generator")]
pub(crate) struct Options {
    /// input JSON file from cargo-license
    #[structopt(short = "i", long = "input")]
    pub(crate) input_file: String,
    /// output dependency report with license information
    #[structopt(short = "o", long = "output")]
    pub(crate) output_file: String,
    /// list of crates to ignore
    #[structopt(short = "g", long = "ignore")]
    pub(crate) ignore_list: Vec<String>,
    #[structopt(short = "t", long = "token")]
    /// github access token
    pub(crate) oauth_token: String,
}
