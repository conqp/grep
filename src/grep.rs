use std::fs::OpenOptions;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

use lines_lossy::LinesLossyExt;
use log::error;
use regex::Regex;

use crate::args::Args;
use crate::files::Files;

#[derive(Clone, Debug)]
pub struct Grep {
    pattern: Regex,
    path: PathBuf,
    recursive: bool,
}

impl Grep {
    #[must_use]
    pub const fn new(pattern: Regex, path: PathBuf, recursive: bool) -> Self {
        Self {
            pattern,
            path,
            recursive,
        }
    }

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

impl From<Args> for Grep {
    fn from(args: Args) -> Self {
        Self::new(args.pattern, args.path, args.recursive)
    }
}
