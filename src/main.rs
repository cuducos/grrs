use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Write};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,

    #[clap(parse(from_os_str), multiple_values = true)]
    paths: Vec<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut stdout = io::BufWriter::new(io::stdout());

    for path in &args.paths {
        if args.paths.len() > 1 {
            writeln!(stdout, "{}:", path.display())?;
        }

        let matches = grrs::find_matches(path, &args.pattern).with_context(|| {
            format!(
                "Error searching for `{}` in {}",
                &args.pattern,
                &path.display()
            )
        })?;

        for line in matches {
            writeln!(stdout, "{}", line)?;
        }

        if args.paths.len() > 1 {
            writeln!(stdout, "")?;
        }
    }

    return Ok(());
}
