use clap::Parser;

use args::Args;
use grep::Grep;

mod args;
mod files;
mod grep;

fn main() -> std::io::Result<()> {
    env_logger::init();
    Grep::from(Args::parse()).run()
}
