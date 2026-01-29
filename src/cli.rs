use clap::{Parser, Subcommand, ValueEnum};
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    Md,
    Json,
    Text,
}
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SortOrder {
    Chronological,
    Reverse,
    Author,
}
#[derive(Debug, Parser)]
#[command(
    name = "rcgen",
    about = "Git changelog generator",
    version,
    long_about = "Generate changelog from git repository with markdown, json, or plaintext output"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand)]
pub enum Commands {
    Gen {
        #[arg(short, long, default_value = ".")]
        path: String,
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Md)]
        format: OutputFormat,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short, long, default_value_t = 0)]
        limit: usize,
        #[arg(long)]
        author: Option<String>,
        #[arg(long)]
        grep: Option<String>,
        #[arg(long)]
        since: Option<String>,
        #[arg(long)]
        until: Option<String>,
        #[arg(long, default_value_t = false)]
        body: bool,
        #[arg(short = 'g', long, default_value_t = false)]
        group: bool,
        #[arg(long, value_enum, default_value_t = SortOrder::Reverse)]
        sort: SortOrder,
        #[arg(long, default_value_t = false)]
        no_merges: bool,
        #[arg(short = 's', long, default_value_t = false)]
        stats: bool,
        #[arg(short = 'r', long)]
        release: bool,
        #[arg(short = 'd', long, default_value_t = false)]
        diff_stats: bool,
    },
    Stats {
        #[arg(short, long, default_value = ".")]
        path: String,
        #[arg(long, default_value_t = false)]
        detailed: bool,
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    Diff {
        #[arg(short, long, default_value = ".")]
        path: String,
        from: String,
        to: Option<String>,
        #[arg(short, long)]
        output: Option<String>,
    },
    Init {
        #[arg(short, long, default_value = ".")]
        path: String,
        #[arg(long, default_value_t = false)]
        force: bool,
    },
    Preview {
        #[arg(short, long, default_value = ".")]
        path: String,
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
}
