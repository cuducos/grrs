use anyhow::{Context, Result};
use clap::Parser;
use std::io;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,

    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).with_context(|| {
        format!(
            "Error reading file: {}",
            &args.path.into_os_string().into_string().unwrap(),
        )
    })?;

    let stdout = io::stdout();
    let writer = io::BufWriter::new(stdout);
    grrs::find_matches(writer, &content, &args.pattern)
        .with_context(|| format!("Error searching for `{}`", &args.pattern))?;

    return Ok(());
}
