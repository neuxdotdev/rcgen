use crate::cli::SortOrder;
use crate::config::Config;
use crate::error::{RcgenError, Result};
use crate::utils;
use chrono::Timelike;
use chrono::{DateTime, FixedOffset};
use git2::{Commit, Repository, Sort};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub short_hash: String,
    pub author: Author,
    pub date: DateTime<FixedOffset>,
    pub message: String,
    pub summary: String,
    pub body: Option<String>,
    pub files_changed: Vec<String>,
    pub insertions: usize,
    pub deletions: usize,
    pub is_merge: bool,
    pub tags: Vec<String>,
    pub branches: Vec<String>,
    pub commit_type: Option<String>,
    pub scope: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
    pub commits_count: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitGroup {
    pub name: String,
    pub commits: Vec<CommitInfo>,
    pub description: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryStats {
    pub total_commits: usize,
    pub total_authors: usize,
    pub first_commit: Option<DateTime<FixedOffset>>,
    pub last_commit: Option<DateTime<FixedOffset>>,
    pub period_days: Option<i64>,
    pub commits_per_day: f64,
    pub authors: Vec<AuthorStats>,
    pub files_changed: usize,
    pub total_insertions: usize,
    pub total_deletions: usize,
    pub bus_factor: f64,
    pub commit_types: HashMap<String, usize>,
    pub most_active_day: Option<String>,
    pub most_active_hour: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorStats {
    pub author: Author,
    pub commits: usize,
    pub insertions: usize,
    pub deletions: usize,
    pub first_commit: DateTime<FixedOffset>,
    pub last_commit: DateTime<FixedOffset>,
    pub commit_types: HashMap<String, usize>,
}
pub struct GitAnalyzer {
    pub repo: Repository,
    pub config: Option<Config>,
}

impl GitAnalyzer {
    pub fn new(path: &str) -> Result<Self> {
        let repo = Repository::open(path)
            .map_err(|e| RcgenError::InvalidPath(format!("{}: {}", path, e)))?;
        let config = Config::load(path)?;
        Ok(Self { repo, config })
    }
    pub fn get_commits(
        &self,
        limit: usize,
        author_filter: Option<&str>,
        grep_filter: Option<&str>,
        since: Option<&str>,
        until: Option<&str>,
        include_body: bool,
        _group_by_type: bool,
        sort_order: SortOrder,
        exclude_merges: bool,
        include_diff_stats: bool,
    ) -> Result<Vec<CommitInfo>> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        match sort_order {
            SortOrder::Chronological => revwalk.set_sorting(Sort::TIME)?,
            SortOrder::Reverse => revwalk.set_sorting(Sort::TIME | Sort::REVERSE)?,
            SortOrder::Author => revwalk.set_sorting(Sort::NONE)?,
        }
        let author_regex = author_filter.map(|f| Regex::new(f)).transpose()?;
        let grep_regex = grep_filter.map(|f| Regex::new(f)).transpose()?;
        let since_time = since.and_then(|s| utils::parse_date(s).ok());
        let until_time = until.and_then(|s| utils::parse_date(s).ok());
        let mut commits = Vec::new();
        let mut count = 0;
        for oid in revwalk {
            let oid = oid?;
            let commit = self.repo.find_commit(oid)?;
            if exclude_merges && commit.parent_count() > 1 {
                continue;
            }
            let commit_info = self.process_commit(&commit, include_body, include_diff_stats)?;
            if let Some(regex) = &author_regex {
                if !regex.is_match(&commit_info.author.name)
                    && !regex.is_match(&commit_info.author.email)
                {
                    continue;
                }
            }
            if let Some(regex) = &grep_regex {
                if !regex.is_match(&commit_info.message) {
                    continue;
                }
            }
            if let Some(since) = since_time {
                if commit_info.date < since {
                    continue;
                }
            }
            if let Some(until) = until_time {
                if commit_info.date > until {
                    continue;
                }
            }
            if let Some(config) = &self.config {
                if config
                    .filters
                    .exclude_authors
                    .contains(&commit_info.author.name)
                    || config
                        .filters
                        .exclude_authors
                        .contains(&commit_info.author.email)
                {
                    continue;
                }
                let mut excluded = false;
                for pattern in &config.filters.exclude_patterns {
                    if let Ok(regex) = Regex::new(pattern) {
                        if regex.is_match(&commit_info.summary) {
                            excluded = true;
                            break;
                        }
                    }
                }
                if excluded {
                    continue;
                }
                if !config.filters.include_patterns.is_empty() {
                    let mut included = false;
                    for pattern in &config.filters.include_patterns {
                        if let Ok(regex) = Regex::new(pattern) {
                            if regex.is_match(&commit_info.summary) {
                                included = true;
                                break;
                            }
                        }
                    }
                    if !included {
                        continue;
                    }
                }
            }
            commits.push(commit_info);
            count += 1;
            if limit > 0 && count >= limit {
                break;
            }
        }
        if let SortOrder::Author = sort_order {
            commits.sort_by(|a, b| a.author.name.cmp(&b.author.name));
        }
        Ok(commits)
    }
    fn process_commit(
        &self,
        commit: &Commit,
        include_body: bool,
        include_diff_stats: bool,
    ) -> Result<CommitInfo> {
        let hash = commit.id().to_string();
        let short_hash = hash.chars().take(8).collect::<String>();
        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown").to_string();
        let author_email = author.email().unwrap_or("unknown").to_string();
        let time = commit.time();
        let date = DateTime::from_timestamp(time.seconds(), 0)
            .ok_or_else(|| RcgenError::DateParse("Invalid timestamp".to_string()))?
            .with_timezone(
                &FixedOffset::east_opt(time.offset_minutes() * 60)
                    .ok_or_else(|| RcgenError::DateParse("Invalid offset".to_string()))?,
            );
        let full_message = commit.message().unwrap_or("").to_string();
        let (summary, body) = if include_body {
            if let Some(pos) = full_message.find("\n\n") {
                (
                    full_message[..pos].trim().to_string(),
                    Some(full_message[pos..].trim().to_string()),
                )
            } else {
                (full_message.trim().to_string(), None)
            }
        } else {
            (
                full_message.lines().next().unwrap_or("").trim().to_string(),
                None,
            )
        };
        let is_merge = commit.parent_count() > 1;
        let (files_changed, insertions, deletions) = if include_diff_stats {
            self.get_commit_stats(commit)?
        } else {
            (Vec::new(), 0, 0)
        };
        let tags = self.get_commit_tags(&hash)?;
        let branches = self.get_commit_branches(commit)?;
        let (commit_type, scope) = self.detect_commit_type(&summary);
        Ok(CommitInfo {
            hash,
            short_hash,
            author: Author {
                name: author_name,
                email: author_email,
                commits_count: 0,
            },
            date,
            message: full_message,
            summary,
            body,
            files_changed,
            insertions,
            deletions,
            is_merge,
            tags,
            branches,
            commit_type,
            scope,
        })
    }
    fn get_commit_stats(&self, commit: &Commit) -> Result<(Vec<String>, usize, usize)> {
        let mut files = Vec::new();
        let mut insertions = 0;
        let mut deletions = 0;
        if commit.parent_count() > 0 {
            let parent = commit.parent(0)?;
            let tree1 = parent.tree()?;
            let tree2 = commit.tree()?;
            let diff = self
                .repo
                .diff_tree_to_tree(Some(&tree1), Some(&tree2), None)?;
            diff.foreach(
                &mut |delta, _| {
                    if let Some(file) = delta.new_file().path() {
                        files.push(file.to_string_lossy().to_string());
                    }
                    true
                },
                None,
                None,
                Some(&mut |_delta, _hunk, line| {
                    match line.origin() {
                        '+' => insertions += 1,
                        '-' => deletions += 1,
                        _ => {}
                    }
                    true
                }),
            )?;
        }
        Ok((files, insertions, deletions))
    }
    fn get_commit_tags(&self, hash: &str) -> Result<Vec<String>> {
        let mut tags = Vec::new();
        let oid = git2::Oid::from_str(hash)?;
        let tag_names = self.repo.tag_names(None)?;
        for name in tag_names.iter().flatten() {
            let obj = self.repo.revparse_single(name)?;
            let peeled = obj.peel(git2::ObjectType::Commit)?;
            if peeled.id() == oid {
                tags.push(name.to_string());
            }
        }
        Ok(tags)
    }
    fn get_commit_branches(&self, commit: &Commit) -> Result<Vec<String>> {
        let mut branches = Vec::new();
        let target_id = commit.id();
        for branch_result in self.repo.branches(None)? {
            let (branch, _) = branch_result?;
            let name = match branch.name()? {
                Some(name) => name.to_string(),
                None => continue,
            };
            let branch_commit = match branch.get().peel_to_commit() {
                Ok(c) => c,
                Err(_) => continue,
            };
            if branch_commit.id() == target_id {
                branches.push(name);
            }
        }
        Ok(branches)
    }
    fn detect_commit_type(&self, summary: &str) -> (Option<String>, Option<String>) {
        let conventional_pattern = Regex::new(r"^(\w+)(?:\(([^)]+)\))?!?:").unwrap();
        if let Some(caps) = conventional_pattern.captures(summary) {
            let commit_type = caps.get(1).map(|m| m.as_str().to_string());
            let scope = caps.get(2).map(|m| m.as_str().to_string());
            return (commit_type, scope);
        }
        let summary_lower = summary.to_lowercase();
        let commit_type = if summary_lower.contains("fix") || summary_lower.contains("bug") {
            Some("fix".to_string())
        } else if summary_lower.contains("feat")
            || summary_lower.contains("add")
            || summary_lower.contains("new")
        {
            Some("feat".to_string())
        } else if summary_lower.contains("doc") {
            Some("docs".to_string())
        } else if summary_lower.contains("test") {
            Some("test".to_string())
        } else if summary_lower.contains("refactor") {
            Some("refactor".to_string())
        } else if summary_lower.contains("perf") {
            Some("perf".to_string())
        } else if summary_lower.contains("chore")
            || summary_lower.contains("ci")
            || summary_lower.contains("build")
        {
            Some("chore".to_string())
        } else {
            None
        };
        (commit_type, None)
    }
    pub fn group_commits(&self, commits: &[CommitInfo]) -> Vec<CommitGroup> {
        let mut groups: HashMap<String, Vec<CommitInfo>> = HashMap::new();
        if let Some(config) = &self.config {
            for group in &config.grouping.groups {
                groups.insert(group.name.clone(), Vec::new());
            }
        }
        let default_groups = vec![
            "Features",
            "Bug Fixes",
            "Documentation",
            "Refactoring",
            "Performance",
            "Tests",
            "Chores",
            "Other",
        ];
        for name in default_groups {
            groups.entry(name.to_string()).or_insert_with(Vec::new);
        }
        for commit in commits {
            let mut placed = false;
            if let Some(config) = &self.config {
                for group in &config.grouping.groups {
                    for pattern in &group.patterns {
                        if let Ok(regex) = Regex::new(pattern) {
                            if regex.is_match(&commit.summary) {
                                groups
                                    .entry(group.name.clone())
                                    .or_default()
                                    .push(commit.clone());
                                placed = true;
                                break;
                            }
                        }
                    }
                    if placed {
                        break;
                    }
                }
            }
            if !placed {
                if let Some(commit_type) = &commit.commit_type {
                    let group_name = match commit_type.as_str() {
                        "feat" => "Features",
                        "fix" => "Bug Fixes",
                        "docs" => "Documentation",
                        "refactor" => "Refactoring",
                        "perf" => "Performance",
                        "test" => "Tests",
                        "chore" => "Chores",
                        _ => "Other",
                    };
                    groups
                        .entry(group_name.to_string())
                        .or_default()
                        .push(commit.clone());
                    placed = true;
                }
            }
            if !placed {
                groups
                    .entry("Other".to_string())
                    .or_default()
                    .push(commit.clone());
            }
        }
        let mut result = Vec::new();
        for (name, commits) in groups {
            if !commits.is_empty() {
                let description = if let Some(config) = &self.config {
                    config
                        .grouping
                        .groups
                        .iter()
                        .find(|g| g.name == name)
                        .and_then(|g| g.description.clone())
                } else {
                    None
                };
                result.push(CommitGroup {
                    name,
                    commits,
                    description,
                });
            }
        }
        let group_order = vec![
            "Features",
            "Bug Fixes",
            "Documentation",
            "Refactoring",
            "Performance",
            "Tests",
            "Chores",
            "Other",
        ];
        result.sort_by(|a, b| {
            let a_index = group_order.iter().position(|&x| x == a.name).unwrap_or(99);
            let b_index = group_order.iter().position(|&x| x == b.name).unwrap_or(99);
            a_index.cmp(&b_index)
        });
        result
    }
    pub fn get_statistics(&self, commits: &[CommitInfo]) -> RepositoryStats {
        let mut authors: HashMap<String, AuthorStats> = HashMap::new();
        let mut commit_types: HashMap<String, usize> = HashMap::new();
        let mut days: HashMap<String, usize> = HashMap::new();
        let mut hours: HashMap<i32, usize> = HashMap::new();
        let mut first_commit = None;
        let mut last_commit = None;
        let mut total_insertions = 0;
        let mut total_deletions = 0;
        let mut files_changed_set = HashSet::new();
        for commit in commits {
            if first_commit.is_none() || commit.date < first_commit.unwrap() {
                first_commit = Some(commit.date);
            }
            if last_commit.is_none() || commit.date > last_commit.unwrap() {
                last_commit = Some(commit.date);
            }
            let author_key = format!("{} <{}>", commit.author.name, commit.author.email);
            let author_entry = authors.entry(author_key).or_insert_with(|| AuthorStats {
                author: commit.author.clone(),
                commits: 0,
                insertions: 0,
                deletions: 0,
                first_commit: commit.date,
                last_commit: commit.date,
                commit_types: HashMap::new(),
            });
            author_entry.commits += 1;
            author_entry.insertions += commit.insertions;
            author_entry.deletions += commit.deletions;
            if commit.date < author_entry.first_commit {
                author_entry.first_commit = commit.date;
            }
            if commit.date > author_entry.last_commit {
                author_entry.last_commit = commit.date;
            }
            if let Some(commit_type) = &commit.commit_type {
                *author_entry
                    .commit_types
                    .entry(commit_type.clone())
                    .or_insert(0) += 1;
                *commit_types.entry(commit_type.clone()).or_insert(0) += 1;
            }
            total_insertions += commit.insertions;
            total_deletions += commit.deletions;
            for file in &commit.files_changed {
                files_changed_set.insert(file.clone());
            }
            let day = commit.date.format("%Y-%m-%d").to_string();
            *days.entry(day).or_insert(0) += 1;
            let hour: i32 = commit.date.hour() as i32;
            *hours.entry(hour).or_insert(0) += 1;
        }
        let mut commit_counts: Vec<usize> = authors.values().map(|a| a.commits).collect();
        commit_counts.sort_by(|a, b| b.cmp(a));
        let mut bus_factor = 0.0;
        let total_commits = commits.len();
        let mut cumulative = 0;
        for count in commit_counts {
            cumulative += count;
            bus_factor += 1.0;
            if cumulative as f64 >= total_commits as f64 * 0.5 {
                break;
            }
        }
        let most_active_day = days
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(day, _)| day.clone());
        let most_active_hour = hours
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(hour, _)| *hour)
            .unwrap_or(0);
        let period_days = if let (Some(first), Some(last)) = (first_commit, last_commit) {
            Some((last - first).num_days())
        } else {
            None
        };
        let commits_per_day = if let Some(days) = period_days {
            if days > 0 {
                commits.len() as f64 / days as f64
            } else {
                commits.len() as f64
            }
        } else {
            0.0
        };
        let mut authors_vec: Vec<AuthorStats> = authors.into_values().collect();
        authors_vec.sort_by(|a, b| b.commits.cmp(&a.commits));
        RepositoryStats {
            total_commits: commits.len(),
            total_authors: authors_vec.len(),
            first_commit,
            last_commit,
            period_days,
            commits_per_day,
            authors: authors_vec,
            files_changed: files_changed_set.len(),
            total_insertions,
            total_deletions,
            bus_factor,
            commit_types,
            most_active_day,
            most_active_hour,
        }
    }
}
