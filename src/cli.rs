use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "lintscout",
    about = "Detect linter ignore directives in source code"
)]
pub struct Cli {
    /// Path to scan
    #[arg(default_value = ".")]
    pub path: PathBuf,

    /// Output format: text, json, count, or sarif
    #[arg(long, default_value = "text")]
    pub format: String,

    /// Path to config file
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// Maximum allowed findings before non-zero exit
    #[arg(long)]
    pub pass_threshold: Option<u64>,

    /// Only run these scouts (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub scouts: Option<Vec<String>>,

    /// Exclude these scouts (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub exclude_scouts: Option<Vec<String>>,

    /// Exclude these paths (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub exclude: Option<Vec<String>>,

    /// Do not respect .gitignore files
    #[arg(long)]
    pub no_gitignore: bool,

    /// Suppress non-essential output
    #[arg(long)]
    pub quiet: bool,
}
