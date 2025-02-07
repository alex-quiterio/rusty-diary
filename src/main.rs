use anyhow::Context;
use rusty_diary::{Config, RustyDiary};
use std::path::PathBuf;
use structopt::StructOpt;
use tracing::info;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "rusty_diary",
    about = "A markdown diary with SQLite persistence",
    author
)]
struct Cli {
    /// Directory containing markdown files
    #[structopt(parse(from_os_str))]
    directory: Option<PathBuf>,

    /// Database file path
    #[structopt(long, parse(from_os_str))]
    db: Option<PathBuf>,

    /// Custom date pattern for files
    #[structopt(long)]
    date_pattern: Option<String>,

    /// Verbosity level
    #[structopt(short, long, parse(from_occurrences))]
    verbose: usize,
}

fn setup_logging(verbosity: usize) {
    let level = match verbosity {
        0 => tracing::Level::INFO,
        1 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(true)
        .with_line_number(true)
        .init();
}

fn build_config(cli: &Cli) -> Config {
    let mut config = Config::new();

    if let Some(dir) = &cli.directory {
        config = config.with_directory(dir);
    }

    if let Some(db) = &cli.db {
        config = config.with_db(db);
    }

    if let Some(pattern) = &cli.date_pattern {
        config = config.with_date_pattern(pattern);
    }

    config
}

async fn run(cli: Cli) -> anyhow::Result<()> {
    info!("Starting Rusty Diary...");

    let config = build_config(&cli);
    info!("Configuration loaded");

    let diary = RustyDiary::new(config)
        .context("Failed to initialize diary")?;

    info!("Processing diary entries...");
    diary.merge()
        .context("Failed to merge entries")?;

    info!("Successfully processed all entries");
    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::from_args();
    setup_logging(cli.verbose);

    if let Err(err) = run(cli).await {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_basic_workflow() -> anyhow::Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_db = temp_dir.path().join("test.db");

        let cli = Cli {
            directory: Some(temp_dir.path().to_path_buf()),
            db: Some(temp_db),
            date_pattern: None,
            verbose: 0,
        };

        let config = build_config(&cli);
        assert_eq!(config.directory, temp_dir.path());
        Ok(())
    }
}