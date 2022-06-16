use anyhow::Result;

pub fn find_matches(mut writer: impl std::io::Write, content: &str, pattern: &str) -> Result<()> {
    for (idx, line) in content.lines().enumerate() {
        if line.contains(pattern) {
            writeln!(writer, "{}: {}", idx + 1, line)?;
        }
    }
    Ok(())
}
