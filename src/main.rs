use clap::Parser;
use rcgen::cli::{Cli, Commands};
use rcgen::error::Result;
fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Gen {
            path,
            format,
            output,
            limit,
            author,
            grep,
            since,
            until,
            body,
            group,
            sort,
            no_merges,
            stats,
            release,
            diff_stats,
        } => {
            let changelog = rcgen::generate_changelog(
                &path,
                format,
                limit,
                author.as_deref(),
                grep.as_deref(),
                since.as_deref(),
                until.as_deref(),
                body,
                group,
                sort,
                no_merges,
                stats,
                diff_stats,
                release,
            )?;
            if let Some(output_path) = output {
                std::fs::write(&output_path, changelog)?;
                println!("Changelog written to {}", output_path);
            } else {
                println!("{}", changelog);
            }
        }
        Commands::Stats {
            path,
            detailed,
            format,
        } => {
            let stats = rcgen::generate_stats(&path, detailed, format)?;
            println!("{}", stats);
        }
        Commands::Init { path, force } => {
            rcgen::init_config(&path, force)?;
            println!("Configuration initialized at {}/.rcgen.toml", path);
        }
        Commands::Preview { path, limit } => {
            let preview = rcgen::preview_changelog(&path, limit)?;
            println!("{}", preview);
        }
        Commands::Diff { .. } => {
            println!("Diff command not implemented yet");
        }
    }
    Ok(())
}
