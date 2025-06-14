use clap::Parser;

use grep::Grep;

mod files;
mod grep;
mod matching_line;

fn main() {
    env_logger::init();
    Grep::parse()
        .run()
        .for_each(|matched_line| println!("{matched_line}"));
}
