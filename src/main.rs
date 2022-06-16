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

    let stdout = io::stdout();
    let writer = io::BufWriter::new(stdout);

    grrs::find_matches(writer, &args.path, &args.pattern).with_context(|| {
        format!(
            "Error searching for `{}` in {}",
            &args.pattern,
            &args.path.display()
        )
    })?;

    return Ok(());
}
