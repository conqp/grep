use std::collections::BTreeSet;
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

use log::trace;

pub struct Files {
    files: BTreeSet<PathBuf>,
    directories: BTreeSet<PathBuf>,
}

impl Files {
    pub fn new(path: PathBuf) -> Self {
        if path.is_dir() {
            Self {
                files: BTreeSet::new(),
                directories: BTreeSet::from([path]),
            }
        } else {
            Self {
                files: BTreeSet::from([path]),
                directories: BTreeSet::new(),
            }
        }
    }
}

impl Iterator for Files {
    type Item = io::Result<PathBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(file) = self.files.pop_first() {
            trace!("Next file: {:?}", &file);
            return Some(Ok(file));
        }

        let directory = self.directories.pop_first()?;

        trace!("Next directory: {:?}", &directory);

        let nodes = match read_dir(&directory) {
            Ok(f) => f,
            Err(e) => return Some(Err(e)),
        };

        for node in nodes.filter_map(Result::ok).map(|entry| entry.path()) {
            if node.is_dir() {
                trace!("Adding directory: {:?}", &node);
                self.directories.insert(node);
            } else {
                trace!("Adding file: {:?}", &node);
                self.files.insert(node);
            }
        }

        self.next()
    }
}
