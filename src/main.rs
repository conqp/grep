use clap::Parser;

use grep::Grep;

mod files;
mod grep;

fn main() -> std::io::Result<()> {
    env_logger::init();
    Grep::parse().run()
}
