pub mod cli;
pub mod config;
pub mod core;
pub mod error;
pub mod utils;

use crate::cli::{OutputFormat, SortOrder};
use crate::core::{CommitInfo, GitAnalyzer};
use crate::error::Result;

pub fn generate_changelog(
    repo_path: &str,
    format: OutputFormat,
    limit: usize,
    author: Option<&str>,
    grep: Option<&str>,
    since: Option<&str>,
    until: Option<&str>,
    include_body: bool,
    group_by_type: bool,
    sort_order: SortOrder,
    exclude_merges: bool,
    include_stats: bool,
    include_diff_stats: bool,
    release_mode: bool,
) -> Result<String> {
    let analyzer = GitAnalyzer::new(repo_path)?;

    let commits = analyzer.get_commits(
        limit,
        author,
        grep,
        since,
        until,
        include_body,
        group_by_type,
        sort_order,
        exclude_merges,
        include_diff_stats,
    )?;

    if commits.is_empty() {
        return match format {
            OutputFormat::Md => Ok("# No commits found\n".to_string()),
            OutputFormat::Json => Ok("[]".to_string()),
            OutputFormat::Text => Ok("No commits found".to_string()),
        };
    }

    let stats = if include_stats {
        Some(analyzer.get_statistics(&commits))
    } else {
        None
    };

    let output = match format {
        OutputFormat::Md => format_markdown(
            &analyzer,
            &commits,
            group_by_type,
            stats.as_ref(),
            release_mode,
        ),
        OutputFormat::Json => format_json(&commits, stats.as_ref())?,
        OutputFormat::Text => format_text(&analyzer, &commits, group_by_type, stats.as_ref()),
    };

    Ok(output)
}

pub fn generate_stats(repo_path: &str, detailed: bool, format: OutputFormat) -> Result<String> {
    let analyzer = GitAnalyzer::new(repo_path)?;
    let commits = analyzer.get_commits(
        0,
        None,
        None,
        None,
        None,
        false,
        false,
        SortOrder::Reverse,
        true,
        false,
    )?;
    let stats = analyzer.get_statistics(&commits);

    match format {
        OutputFormat::Md => Ok(format_stats_markdown(&stats, detailed)),
        OutputFormat::Json => Ok(format_stats_json(&stats, detailed)?),
        OutputFormat::Text => Ok(format_stats_text(&stats, detailed)),
    }
}

pub fn init_config(repo_path: &str, force: bool) -> Result<()> {
    crate::config::Config::init(repo_path, force)
}

pub fn preview_changelog(repo_path: &str, limit: usize) -> Result<String> {
    let analyzer = GitAnalyzer::new(repo_path)?;
    let commits = analyzer.get_commits(
        limit,
        None,
        None,
        None,
        None,
        false,
        true,
        SortOrder::Reverse,
        true,
        false,
    )?;

    let mut output = String::new();
    output.push_str("Preview of last commits:\n\n");

    for (i, commit) in commits.iter().enumerate() {
        output.push_str(&format!(
            "{}. {} - {}\n",
            i + 1,
            commit.short_hash,
            commit.summary
        ));
        output.push_str(&format!(
            "   Author: {} <{}>\n",
            commit.author.name, commit.author.email
        ));
        output.push_str(&format!(
            "   Date: {}\n",
            commit.date.format("%Y-%m-%d %H:%M:%S")
        ));

        if !commit.files_changed.is_empty() {
            output.push_str(&format!(
                "   Files: {} changed (+{} -{})\n",
                commit.files_changed.len(),
                commit.insertions,
                commit.deletions
            ));
        }

        if !commit.tags.is_empty() {
            output.push_str(&format!("   Tags: {}\n", commit.tags.join(", ")));
        }

        output.push('\n');
    }

    Ok(output)
}

fn format_markdown(
    analyzer: &GitAnalyzer,
    commits: &[CommitInfo],
    group_by_type: bool,
    stats: Option<&crate::core::RepositoryStats>,
    release_mode: bool,
) -> String {
    let mut output = String::new();

    // Header
    if let Some(config) = &analyzer.config {
        if let Some(header) = &config.templates.header {
            output.push_str(header);
            output.push('\n');
        }
    } else {
        output.push_str("# Changelog\n\n");
    }

    if release_mode {
        output.push_str("## Release Notes\n\n");
    }

    // Group commits or list them
    if group_by_type {
        let groups = analyzer.group_commits(commits);

        for group in groups {
            output.push_str(&format!("### {}\n", group.name));

            if let Some(desc) = &group.description {
                output.push_str(&format!("{}\n\n", desc));
            }

            for commit in &group.commits {
                output.push_str(&format_markdown_commit(commit));
            }

            output.push('\n');
        }
    } else {
        // Group by date
        let mut commits_by_date: std::collections::BTreeMap<String, Vec<&CommitInfo>> =
            std::collections::BTreeMap::new();

        for commit in commits {
            let date_key = commit.date.format("%Y-%m-%d").to_string();
            commits_by_date.entry(date_key).or_default().push(commit);
        }

        for (date, date_commits) in commits_by_date.iter().rev() {
            output.push_str(&format!("## {}\n\n", date));

            for commit in date_commits {
                output.push_str(&format_markdown_commit(commit));
            }

            output.push('\n');
        }
    }

    // Statistics
    if let Some(stats) = stats {
        output.push_str("## Statistics\n\n");
        output.push_str(&format!("- Total commits: {}\n", stats.total_commits));
        output.push_str(&format!("- Total authors: {}\n", stats.total_authors));
        output.push_str(&format!("- Files changed: {}\n", stats.files_changed));
        output.push_str(&format!("- Insertions: +{}\n", stats.total_insertions));
        output.push_str(&format!("- Deletions: -{}\n", stats.total_deletions));
        output.push_str(&format!("- Bus factor: {:.1}\n", stats.bus_factor));

        if let Some(first) = stats.first_commit {
            if let Some(last) = stats.last_commit {
                output.push_str(&format!(
                    "- Period: {} to {} ({} days)\n",
                    first.format("%Y-%m-%d"),
                    last.format("%Y-%m-%d"),
                    stats.period_days.unwrap_or(0)
                ));
                output.push_str(&format!(
                    "- Commits per day: {:.2}\n",
                    stats.commits_per_day
                ));
            }
        }

        output.push('\n');

        // Top authors
        if !stats.authors.is_empty() {
            output.push_str("### Top Contributors\n\n");
            for (i, author) in stats.authors.iter().take(5).enumerate() {
                output.push_str(&format!(
                    "{}. {} <{}> - {} commits (+{} -{})\n",
                    i + 1,
                    author.author.name,
                    author.author.email,
                    author.commits,
                    author.insertions,
                    author.deletions
                ));
            }
            output.push('\n');
        }
    }

    // Footer
    if let Some(config) = &analyzer.config {
        if let Some(footer) = &config.templates.footer {
            output.push_str(footer);
            output.push('\n');
        }
    }

    output
}

fn format_markdown_commit(commit: &CommitInfo) -> String {
    let mut line = String::new();

    line.push_str("- ");

    // Format based on commit type
    if let Some(commit_type) = &commit.commit_type {
        let emoji = match commit_type.as_str() {
            "feat" => "✨",
            "fix" => "🐛",
            "docs" => "📚",
            "refactor" => "♻️",
            "perf" => "⚡",
            "test" => "✅",
            "chore" => "🔧",
            _ => "📝",
        };
        line.push_str(&format!("{} ", emoji));
    }

    line.push_str(&commit.summary);

    // Add hash and author
    line.push_str(&format!(" (`{}`", commit.short_hash));

    if !commit.tags.is_empty() {
        line.push_str(&format!(", tags: {}", commit.tags.join(", ")));
    }

    line.push_str(&format!(" by {})", commit.author.name));

    // Add diff stats if available
    if !commit.files_changed.is_empty() {
        line.push_str(&format!(
            " - {} files changed (+{} -{})",
            commit.files_changed.len(),
            commit.insertions,
            commit.deletions
        ));
    }

    line.push('\n');

    let mut output = String::new();

    if let Some(body) = &commit.body {
        if !body.trim().is_empty() {
            for l in body.lines() {
                let trimmed = l.trim();
                if !trimmed.is_empty() {
                    output.push_str(&format!("  > {}\n", trimmed));
                }
            }
        }
    }

    line
}

fn format_json(
    commits: &[CommitInfo],
    stats: Option<&crate::core::RepositoryStats>,
) -> Result<String> {
    #[derive(serde::Serialize)]
    struct Output {
        commits: Vec<CommitInfo>,
        stats: Option<crate::core::RepositoryStats>,
        generated_at: chrono::DateTime<chrono::Utc>,
    }

    let output = Output {
        commits: commits.to_vec(),
        stats: stats.cloned(),
        generated_at: chrono::Utc::now(),
    };

    serde_json::to_string_pretty(&output).map_err(Into::into)
}

fn format_text(
    analyzer: &GitAnalyzer,
    commits: &[CommitInfo],
    group_by_type: bool,
    stats: Option<&crate::core::RepositoryStats>,
) -> String {
    let mut output = String::new();

    output.push_str("CHANGELOG\n");
    output.push_str(&"=".repeat(80));
    output.push('\n');

    if group_by_type {
        let groups = analyzer.group_commits(commits);

        for group in groups {
            output.push_str(&format!("\n{}\n", group.name.to_uppercase()));
            output.push_str(&"-".repeat(group.name.len()));
            output.push('\n');

            if let Some(desc) = &group.description {
                output.push_str(&format!("{}\n\n", desc));
            }

            for commit in &group.commits {
                output.push_str(&format_text_commit(commit));
            }
        }
    } else {
        // Group by date
        let mut commits_by_date: std::collections::BTreeMap<String, Vec<&CommitInfo>> =
            std::collections::BTreeMap::new();

        for commit in commits {
            let date_key = commit.date.format("%Y-%m-%d").to_string();
            commits_by_date.entry(date_key).or_default().push(commit);
        }

        for (date, date_commits) in commits_by_date.iter().rev() {
            output.push_str(&format!("\n{}\n", date));
            output.push_str(&"-".repeat(date.len()));
            output.push('\n');

            for commit in date_commits {
                output.push_str(&format_text_commit(commit));
            }
        }
    }

    if let Some(stats) = stats {
        output.push_str("\nSTATISTICS\n");
        output.push_str(&"-".repeat(80));
        output.push('\n');

        output.push_str(&format!("Total commits: {}\n", stats.total_commits));
        output.push_str(&format!("Total authors: {}\n", stats.total_authors));
        output.push_str(&format!("Files changed: {}\n", stats.files_changed));
        output.push_str(&format!("Insertions: +{}\n", stats.total_insertions));
        output.push_str(&format!("Deletions: -{}\n", stats.total_deletions));
        output.push_str(&format!("Bus factor: {:.1}\n", stats.bus_factor));

        if let Some(first) = stats.first_commit {
            if let Some(last) = stats.last_commit {
                output.push_str(&format!(
                    "Period: {} to {} ({} days)\n",
                    first.format("%Y-%m-%d"),
                    last.format("%Y-%m-%d"),
                    stats.period_days.unwrap_or(0)
                ));
                output.push_str(&format!("Commits per day: {:.2}\n", stats.commits_per_day));
            }
        }

        if !stats.authors.is_empty() {
            output.push_str("\nTop contributors:\n");
            for (i, author) in stats.authors.iter().take(5).enumerate() {
                output.push_str(&format!(
                    "  {}. {} <{}> - {} commits (+{} -{})\n",
                    i + 1,
                    author.author.name,
                    author.author.email,
                    author.commits,
                    author.insertions,
                    author.deletions
                ));
            }
        }
    }

    output
}

fn format_text_commit(commit: &CommitInfo) -> String {
    let mut line = String::new();

    line.push_str(&format!("* {}", commit.summary));

    // Add hash and author
    line.push_str(&format!(" [{}]", commit.short_hash));
    line.push_str(&format!(" - {}", commit.author.name));

    // Add date
    line.push_str(&format!(" ({})", commit.date.format("%Y-%m-%d")));

    // Add diff stats if available
    if !commit.files_changed.is_empty() {
        line.push_str(&format!(
            " | {} files, +{}/-{}",
            commit.files_changed.len(),
            commit.insertions,
            commit.deletions
        ));
    }

    // Add tags if present
    if !commit.tags.is_empty() {
        line.push_str(&format!(" | tags: {}", commit.tags.join(", ")));
    }

    line.push('\n');

    // Add body if present
    if let Some(body) = &commit.body {
        if !body.trim().is_empty() {
            for line_body in body.lines().take(3) {
                // Limit body lines
                if !line_body.trim().is_empty() {
                    line.push_str(&format!("    > {}\n", line_body.trim()));
                }
            }
        }
    }

    line
}

fn format_stats_markdown(stats: &crate::core::RepositoryStats, detailed: bool) -> String {
    let mut output = String::new();

    output.push_str("# Repository Statistics\n\n");

    output.push_str("## Summary\n\n");
    output.push_str(&format!("- **Total commits:** {}\n", stats.total_commits));
    output.push_str(&format!("- **Total authors:** {}\n", stats.total_authors));
    output.push_str(&format!("- **Files changed:** {}\n", stats.files_changed));
    output.push_str(&format!(
        "- **Total changes:** +{} / -{}\n",
        stats.total_insertions, stats.total_deletions
    ));
    output.push_str(&format!("- **Bus factor:** {:.1}\n", stats.bus_factor));

    if let Some(first) = stats.first_commit {
        if let Some(last) = stats.last_commit {
            output.push_str(&format!(
                "- **Period:** {} to {} ({} days)\n",
                first.format("%Y-%m-%d"),
                last.format("%Y-%m-%d"),
                stats.period_days.unwrap_or(0)
            ));
            output.push_str(&format!(
                "- **Commits per day:** {:.2}\n",
                stats.commits_per_day
            ));
        }
    }

    if let Some(day) = &stats.most_active_day {
        output.push_str(&format!("- **Most active day:** {}\n", day));
    }

    output.push_str(&format!(
        "- **Most active hour:** {:02}:00\n",
        stats.most_active_hour
    ));

    if detailed {
        output.push_str("\n## Authors\n\n");

        for (i, author) in stats.authors.iter().enumerate() {
            output.push_str(&format!(
                "### {}. {} <{}>\n",
                i + 1,
                author.author.name,
                author.author.email
            ));
            output.push_str(&format!(
                "- **Commits:** {} ({:.1}% of total)\n",
                author.commits,
                (author.commits as f32 / stats.total_commits as f32) * 100.0
            ));
            output.push_str(&format!(
                "- **Changes:** +{} / -{}\n",
                author.insertions, author.deletions
            ));
            output.push_str(&format!(
                "- **First commit:** {}\n",
                author.first_commit.format("%Y-%m-%d")
            ));
            output.push_str(&format!(
                "- **Last commit:** {}\n",
                author.last_commit.format("%Y-%m-%d")
            ));

            if !author.commit_types.is_empty() {
                output.push_str("- **Commit types:** ");
                let types: Vec<String> = author
                    .commit_types
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                output.push_str(&types.join(", "));
                output.push('\n');
            }

            output.push('\n');
        }

        if !stats.commit_types.is_empty() {
            output.push_str("## Commit Types\n\n");

            let mut types: Vec<(&String, &usize)> = stats.commit_types.iter().collect();
            types.sort_by(|a, b| b.1.cmp(a.1));

            for (commit_type, count) in types {
                output.push_str(&format!(
                    "- **{}:** {} ({:.1}%)\n",
                    commit_type,
                    count,
                    (*count as f32 / stats.total_commits as f32) * 100.0
                ));
            }

            output.push('\n');
        }
    } else {
        output.push_str("\n## Top Contributors\n\n");

        for (i, author) in stats.authors.iter().take(10).enumerate() {
            output.push_str(&format!(
                "{}. **{}** - {} commits (+{} / -{})\n",
                i + 1,
                author.author.name,
                author.commits,
                author.insertions,
                author.deletions
            ));
        }
    }

    output
}

fn format_stats_json(stats: &crate::core::RepositoryStats, detailed: bool) -> Result<String> {
    #[derive(serde::Serialize)]
    struct Output {
        summary: SummaryStats,
        authors: Vec<crate::core::AuthorStats>,
        commit_types: std::collections::HashMap<String, usize>,
        generated_at: chrono::DateTime<chrono::Utc>,
    }

    #[derive(serde::Serialize)]
    struct SummaryStats {
        total_commits: usize,
        total_authors: usize,
        files_changed: usize,
        total_insertions: usize,
        total_deletions: usize,
        bus_factor: f64,
        first_commit: Option<String>,
        last_commit: Option<String>,
        period_days: Option<i64>,
        commits_per_day: f64,
        most_active_day: Option<String>,
        most_active_hour: i32,
    }

    let output = Output {
        summary: SummaryStats {
            total_commits: stats.total_commits,
            total_authors: stats.total_authors,
            files_changed: stats.files_changed,
            total_insertions: stats.total_insertions,
            total_deletions: stats.total_deletions,
            bus_factor: stats.bus_factor,
            first_commit: stats.first_commit.map(|d| d.to_rfc3339()),
            last_commit: stats.last_commit.map(|d| d.to_rfc3339()),
            period_days: stats.period_days,
            commits_per_day: stats.commits_per_day,
            most_active_day: stats.most_active_day.clone(),
            most_active_hour: stats.most_active_hour,
        },
        authors: if detailed {
            stats.authors.clone()
        } else {
            stats.authors.iter().take(10).cloned().collect()
        },
        commit_types: stats.commit_types.clone(),
        generated_at: chrono::Utc::now(),
    };

    serde_json::to_string_pretty(&output).map_err(Into::into)
}

fn format_stats_text(stats: &crate::core::RepositoryStats, detailed: bool) -> String {
    let mut output = String::new();

    output.push_str("REPOSITORY STATISTICS\n");
    output.push_str(&"=".repeat(80));
    output.push('\n');

    output.push_str(&format!("Total commits:      {}\n", stats.total_commits));
    output.push_str(&format!("Total authors:      {}\n", stats.total_authors));
    output.push_str(&format!("Files changed:      {}\n", stats.files_changed));
    output.push_str(&format!(
        "Total changes:      +{} / -{}\n",
        stats.total_insertions, stats.total_deletions
    ));
    output.push_str(&format!("Bus factor:         {:.1}\n", stats.bus_factor));

    if let Some(first) = stats.first_commit {
        if let Some(last) = stats.last_commit {
            output.push_str(&format!(
                "Period:             {} to {} ({} days)\n",
                first.format("%Y-%m-%d"),
                last.format("%Y-%m-%d"),
                stats.period_days.unwrap_or(0)
            ));
            output.push_str(&format!(
                "Commits per day:    {:.2}\n",
                stats.commits_per_day
            ));
        }
    }

    if let Some(day) = &stats.most_active_day {
        output.push_str(&format!("Most active day:    {}\n", day));
    }

    output.push_str(&format!(
        "Most active hour:   {:02}:00\n",
        stats.most_active_hour
    ));

    output.push_str("\n");
    output.push_str("TOP CONTRIBUTORS\n");
    output.push_str(&"-".repeat(80));
    output.push('\n');

    for (i, author) in stats.authors.iter().take(10).enumerate() {
        output.push_str(&format!(
            "{:2}. {:<20} {:>4} commits  +{:>6}/-{:>6}\n",
            i + 1,
            if author.author.name.len() > 20 {
                format!("{}...", &author.author.name[..17])
            } else {
                author.author.name.clone()
            },
            author.commits,
            author.insertions,
            author.deletions
        ));
    }

    if detailed {
        output.push_str("\n");
        output.push_str("DETAILED STATISTICS\n");
        output.push_str(&"-".repeat(80));
        output.push('\n');

        for (i, author) in stats.authors.iter().enumerate() {
            output.push_str(&format!(
                "\n{}. {} <{}>\n",
                i + 1,
                author.author.name,
                author.author.email
            ));
            output.push_str(&format!(
                "   Commits: {} ({:.1}% of total)\n",
                author.commits,
                (author.commits as f32 / stats.total_commits as f32) * 100.0
            ));
            output.push_str(&format!(
                "   Changes: +{} / -{}\n",
                author.insertions, author.deletions
            ));
            output.push_str(&format!(
                "   First commit: {}\n",
                author.first_commit.format("%Y-%m-%d")
            ));
            output.push_str(&format!(
                "   Last commit: {}\n",
                author.last_commit.format("%Y-%m-%d")
            ));

            if !author.commit_types.is_empty() {
                output.push_str("   Commit types: ");
                let types: Vec<String> = author
                    .commit_types
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect();
                output.push_str(&types.join(", "));
                output.push('\n');
            }
        }

        if !stats.commit_types.is_empty() {
            output.push_str("\n");
            output.push_str("COMMIT TYPES\n");
            output.push_str(&"-".repeat(80));
            output.push('\n');

            let mut types: Vec<(&String, &usize)> = stats.commit_types.iter().collect();
            types.sort_by(|a, b| b.1.cmp(a.1));

            for (commit_type, count) in types {
                output.push_str(&format!(
                    "{:<12} {:>4} ({:5.1}%)\n",
                    commit_type,
                    count,
                    (*count as f32 / stats.total_commits as f32) * 100.0
                ));
            }
        }
    }

    output
}
