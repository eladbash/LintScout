use std::process;

use clap::Parser;

use lintscout::cli::Cli;
use lintscout::config::Config;
use lintscout::output;
use lintscout::registry::ScoutRegistry;
use lintscout::scanner::Scanner;

fn main() {
    let cli = Cli::parse();

    let config = if let Some(ref path) = cli.config {
        match Config::load(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error loading config: {e}");
                process::exit(2);
            }
        }
    } else {
        match Config::find_and_load() {
            Some(Ok(c)) => c,
            Some(Err(e)) => {
                eprintln!("Error loading config: {e}");
                process::exit(2);
            }
            None => Config::default(),
        }
    };

    let format = if cli.format != "text" {
        cli.format.clone()
    } else {
        config.settings.output.clone()
    };

    let pass_threshold = cli.pass_threshold.or(config.settings.pass_threshold);

    let excludes = cli
        .exclude
        .clone()
        .unwrap_or_else(|| config.settings.exclude.clone());

    let respect_gitignore = !cli.no_gitignore && config.settings.respect_gitignore;

    let registry = match ScoutRegistry::new().with_builtins() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error initializing scouts: {e}");
            process::exit(2);
        }
    };

    let registry = match registry.with_config(&config) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error loading custom scouts: {e}");
            process::exit(2);
        }
    };

    let mut exclude_scouts = config.settings.disable.scouts.clone();
    if let Some(ref cli_excludes) = cli.exclude_scouts {
        exclude_scouts.extend(cli_excludes.iter().cloned());
    }

    let registry = if let Some(ref names) = cli.scouts {
        registry.filter(names)
    } else {
        registry
    };

    let scouts = registry.exclude(&exclude_scouts).into_scouts();

    if scouts.is_empty() {
        if !cli.quiet {
            eprintln!("No scouts selected. Check your --scouts or --exclude-scouts flags.");
        }
        process::exit(0);
    }

    let scanner = Scanner::new(&cli.path, scouts)
        .with_excludes(excludes)
        .with_gitignore(respect_gitignore);

    let result = match scanner.run() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Scan error: {e}");
            process::exit(2);
        }
    };

    if !cli.quiet || !result.findings.is_empty() {
        print!("{}", output::format_output(&result, &format));
    }

    let exit_code = match pass_threshold {
        Some(threshold) => {
            if result.stats.findings_count > threshold {
                1
            } else {
                0
            }
        }
        None => {
            if result.stats.findings_count > 0 {
                1
            } else {
                0
            }
        }
    };

    process::exit(exit_code);
}
