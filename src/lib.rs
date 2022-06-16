use anyhow::{Context, Result};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Match {
    line_number: usize,
    content: String,
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.line_number, self.content)
    }
}

pub fn find_matches(path: &std::path::PathBuf, pattern: &str) -> Result<Vec<Match>> {
    let file = File::open(path).with_context(|| format!("Error opening {}", path.display()))?;
    let reader = BufReader::new(file);
    let mut matches: Vec<Match> = Vec::new();

    for (idx, result) in reader.lines().enumerate() {
        let number = idx + 1;
        let line = result
            .with_context(|| format!("Error reading line {} from {}", number, path.display()))?;
        if line.contains(pattern) {
            matches.push(Match {
                line_number: number,
                content: line,
            });
        }
    }

    Ok(matches)
}
