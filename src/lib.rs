use anyhow::Result;

pub fn find_matches(mut writer: impl std::io::Write, content: &str, pattern: &str) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}
