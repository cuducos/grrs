use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn find_matches(
    mut writer: impl std::io::Write,
    path: &std::path::PathBuf,
    pattern: &str,
) -> Result<()> {
    let file = File::open(path).with_context(|| format!("Error opening {}", path.display()))?;
    let reader = BufReader::new(file);
    for (idx, result) in reader.lines().enumerate() {
        let line = result
            .with_context(|| format!("Error reading line {} from {}", idx + 1, path.display()))?;
        if line.contains(pattern) {
            writeln!(writer, "{}: {}", idx + 1, line)?;
        }
    }

    Ok(())
}
