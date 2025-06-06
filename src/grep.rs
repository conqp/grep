use std::fs::OpenOptions;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

use clap::Parser;
use lines_lossy::LinesLossyExt;
use log::error;
use regex::Regex;

use crate::files::Files;

#[derive(Clone, Debug, Parser)]
pub struct Grep {
    #[clap(index = 1)]
    pattern: Regex,
    #[clap(index = 2, default_value = "./")]
    path: PathBuf,
    #[clap(short, long)]
    recursive: bool,
}

impl Grep {
    pub fn run(&self) -> io::Result<()> {
        if self.recursive {
            for path in Files::new(&self.path)
                .filter_map(|path| path.inspect_err(|error| error!("{error}")).ok())
            {
                match self.grep(&path) {
                    Ok(()) => (),
                    Err(error) => {
                        error!("{path:?}: {error}");
                    }
                }
            }

            Ok(())
        } else {
            self.grep(&self.path)
        }
    }

    fn grep(&self, path: &Path) -> io::Result<()> {
        BufReader::new(OpenOptions::new().read(true).open(path)?)
            .lines_lossy()
            .enumerate()
            .filter_map(|(i, line)| line.ok().map(|line| (i, line)))
            .for_each(|(index, line)| {
                if self.pattern.is_match(&line) {
                    println!("{path:?}: #{index} {line}");
                }
            });

        Ok(())
    }
}
