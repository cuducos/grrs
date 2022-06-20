use anyhow::Result;
use camino::Utf8PathBuf as PathBuf;
use clap::Parser;
use std::io::{self, Write};
use tokio::sync::mpsc;

/// Search for a pattern in file(s) and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,

    #[clap(multiple_values = true)]
    paths: Vec<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Cli::parse();
    let mut stdout = io::BufWriter::new(io::stdout());
    let paths = args.paths;
    let multiple_paths = paths.len() > 1;
    let (tx, mut rx) = mpsc::channel(64);

    tokio::spawn(async move {
        for path in &paths {
            let matches = grrs::find_matches(path, &args.pattern).await;
            tx.send(matches).await.unwrap();
        }
    });

    while let Some(result) = rx.recv().await {
        let matches = result?;
        if multiple_paths {
            writeln!(stdout, "{}:", matches.path)?;
        }

        for line in matches.lines {
            writeln!(stdout, "{}", line)?;
        }

        if multiple_paths {
            writeln!(stdout, "")?;
        }
    }

    return Ok(());
}
