use clap::{Parser, Subcommand};
use walkdir::WalkDir;
use std::{
    fs,
    path::PathBuf,
    time::{Duration, SystemTime},
};

#[derive(Parser)]
#[command(name = "brewclean")]
#[command(about = "Clean Homebrew downloads cache")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// List files that would be removed
    Ls {
        #[arg(long, default_value = "0")]
        keep_days: u64,
    },

    /// Clean downloads cache
    Clean {
        #[arg(long, default_value = "0")]
        keep_days: u64,
    },
}

fn main() {
    let cli = Cli::parse();
    let downloads_dir = downloads_dir();

    if !downloads_dir.exists() {
        eprintln!("Not found: {}", downloads_dir.display());
        std::process::exit(1);
    }

    match cli.command {
        Command::Ls { keep_days } => {
            for file in collect_files(&downloads_dir, keep_days) {
                println!("{}", file.display());
            }
        }
        Command::Clean { keep_days } => {
            for file in collect_files(&downloads_dir, keep_days) {
                let _ = fs::remove_file(file);
            }
        }
    }
}

fn collect_files(dir: &PathBuf, keep_days: u64) -> Vec<PathBuf> {
    let cutoff = if keep_days > 0 {
        Some(SystemTime::now() - Duration::from_secs(keep_days * 86400))
    } else {
        None
    };

    WalkDir::new(dir)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            if let Some(cutoff) = cutoff {
                if let Ok(meta) = e.metadata() {
                    if let Ok(modified) = meta.modified() {
                        return modified <= cutoff;
                    }
                }
            }
            true
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn downloads_dir() -> PathBuf {
    PathBuf::from(std::env::var("HOME").expect("HOME not set"))
        .join("Library")
        .join("Caches")
        .join("Homebrew")
        .join("downloads")
}