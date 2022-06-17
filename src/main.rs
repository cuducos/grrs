use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Parser;
use log::warn;
use std::io::{self, Write};
use tokio::sync::mpsc;

/// Search for a pattern in file(s) and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,

    #[clap(parse(from_os_str), multiple_values = true)]
    paths: Vec<std::path::PathBuf>,
}

fn convert_paths_to_utf8(paths: Vec<std::path::PathBuf>) -> Vec<Utf8PathBuf> {
    let mut results: Vec<Utf8PathBuf> = Vec::new();
    let mut errors = false;
    for path in paths {
        match Utf8PathBuf::from_path_buf(path) {
            Ok(value) => results.push(value),
            _ => errors = true,
        }
    }
    if errors {
        warn!(
            "Discarded some file paths because they had ivalid encoding. Valid file paths: {}",
            results.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ")
        );
    }
    results
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let mut stdout = io::BufWriter::new(io::stdout());
    let paths = convert_paths_to_utf8(args.paths);
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
