use std::fmt::Display;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchingLine {
    filename: PathBuf,
    line: usize,
    text: String,
}

impl MatchingLine {
    #[must_use]
    pub const fn new(filename: PathBuf, line: usize, text: String) -> Self {
        Self {
            filename,
            line,
            text,
        }
    }

    #[must_use]
    pub fn filename(&self) -> &Path {
        &self.filename
    }

    #[must_use]
    pub fn line(&self) -> usize {
        self.line
    }

    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Display for MatchingLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Line #{} in {:?}: {}",
            self.line(),
            self.filename(),
            self.text()
        )
    }
}
