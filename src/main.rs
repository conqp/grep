use clap::Parser;

use args::Args;

mod args;
mod files;

fn main() -> std::io::Result<()> {
    env_logger::init();
    Args::parse().run()
}
