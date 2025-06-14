use std::fs::OpenOptions;
use std::io::{self, BufReader};
use std::iter::once;
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
            if let Some(files) = Files::new(&self.path) {
                self.grep_recursive(files)
            } else {
                self.grep_recursive(once(Ok(self.path.clone())))
            }
        } else {
            self.grep(&self.path)
        }
    }

    fn grep_recursive(&self, files: impl Iterator<Item = io::Result<PathBuf>>) -> io::Result<()> {
        for file in files.filter_map(|path| path.inspect_err(|error| error!("{error}")).ok()) {
            match self.grep(&file) {
                Ok(()) => (),
                Err(error) => {
                    error!("{file:?}: {error}");
                }
            }
        }

        Ok(())
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
