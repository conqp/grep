use std::path::PathBuf;

use clap::Parser;
use regex::Regex;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(index = 1)]
    pub(crate) pattern: Regex,
    #[clap(index = 2, default_value = "./")]
    pub(crate) path: PathBuf,
    #[clap(short, long)]
    pub(crate) recursive: bool,
}
