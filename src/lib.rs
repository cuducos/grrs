use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use std::fmt;

use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

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
        write!(f, "{} has {} match(es)", self.path, self.lines.len())
    }
}

#[tracing::instrument]
pub async fn find_matches(path: &Utf8PathBuf, pattern: &str) -> Result<Matches> {
    let path_as_str = path.to_string();
    let file = File::open(path)
        .await
        .with_context(|| format!("Error opening {}", &path_as_str))?;
    let reader = BufReader::new(file);
    let mut matches = Matches {
        path: path.to_string(),
        lines: Vec::new(),
    };

    let mut number = 0;
    let mut lines = reader.lines();

    while let Some(line) = lines
        .next_line()
        .await
        .with_context(|| format!("Error reading line {} from {}", number, &path_as_str))?
    {
        number += 1;
        if line.contains(pattern) {
            matches.lines.push(Match {
                line_number: number,
                content: line,
            });
        }
    }

    Ok(matches)
}
