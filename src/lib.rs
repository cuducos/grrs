use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Match {
    pub line_number: usize,
    pub content: String,
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.line_number, self.content)
    }
}

pub struct Matches {
    pub path: String,
    pub lines: Vec<Match>,
}

impl fmt::Debug for Matches {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} has {} matche(s)", self.path, self.lines.len())
    }
}

pub async fn find_matches(path: &Utf8PathBuf, pattern: &str) -> Result<Matches> {
    let path_as_str = path.to_string();
    let file = File::open(path).with_context(|| format!("Error opening {}", &path_as_str))?;
    let reader = BufReader::new(file);
    let mut matches = Matches {
        path: path.to_string(),
        lines: Vec::new(),
    };

    for (idx, result) in reader.lines().enumerate() {
        let number = idx + 1;
        let line = result
            .with_context(|| format!("Error reading line {} from {}", number, &path_as_str))?;
        if line.contains(pattern) {
            matches.lines.push(Match {
                line_number: number,
                content: line,
            });
        }
    }

    Ok(matches)
}
