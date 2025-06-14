use std::fs::OpenOptions;
use std::io::{self, BufReader};
use std::iter::once;
use std::path::{Path, PathBuf};

use clap::Parser;
use lines_lossy::LinesLossyExt;
use log::error;
use regex::Regex;

use crate::files::Files;
use crate::matching_line::MatchingLine;

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
    pub fn run(&self) -> Box<dyn Iterator<Item = MatchingLine> + '_> {
        if self.recursive {
            if let Some(files) = Files::new(&self.path) {
                Box::new(self.grep_recursive(files))
            } else {
                Box::new(self.grep_recursive(once(Ok(self.path.clone()))))
            }
        } else {
            Box::new(self.grep(&self.path))
        }
    }

    fn grep_recursive(
        &self,
        files: impl Iterator<Item = io::Result<PathBuf>>,
    ) -> impl Iterator<Item = MatchingLine> {
        files
            .filter_map(|path| path.inspect_err(|error| error!("{error}")).ok())
            .flat_map(|file| self.grep(file))
    }

    fn grep(&self, path: impl AsRef<Path>) -> impl Iterator<Item = MatchingLine> {
        OpenOptions::new()
            .read(true)
            .open(path.as_ref())
            .ok()
            .into_iter()
            .flat_map(move |file| {
                BufReader::new(file)
                    .lines_lossy()
                    .enumerate()
                    .filter_map(|(i, line)| line.ok().map(|line| (i, line)))
                    .filter_map({
                        let value = path.as_ref().to_path_buf();
                        move |(index, line)| {
                            if self.pattern.is_match(&line) {
                                Some(MatchingLine::new(value.clone(), index, line))
                            } else {
                                None
                            }
                        }
                    })
            })
    }
}
