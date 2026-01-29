use crate::error::{RcgenError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub repository: RepositoryConfig,
    pub output: OutputConfig,
    pub filters: FilterConfig,
    pub grouping: GroupingConfig,
    pub templates: TemplateConfig,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RepositoryConfig {
    pub url: Option<String>,
    pub default_branch: String,
    pub tag_pattern: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputConfig {
    pub default_format: String,
    pub include_body: bool,
    pub include_diff_stats: bool,
    pub exclude_merges: bool,
    pub max_commits: usize,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterConfig {
    pub exclude_authors: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub include_patterns: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupingConfig {
    pub enabled: bool,
    pub groups: Vec<CommitGroup>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommitGroup {
    pub name: String,
    pub patterns: Vec<String>,
    pub description: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateConfig {
    pub header: Option<String>,
    pub footer: Option<String>,
    pub commit_format: String,
    pub date_format: String,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            repository: RepositoryConfig {
                url: None,
                default_branch: "main".to_string(),
                tag_pattern: Some("v[0-9]*".to_string()),
            },
            output: OutputConfig {
                default_format: "md".to_string(),
                include_body: false,
                include_diff_stats: true,
                exclude_merges: true,
                max_commits: 100,
            },
            filters: FilterConfig {
                exclude_authors: vec![],
                exclude_patterns: vec![
                    "^Merge".to_string(),
                    "^Revert".to_string(),
                ],
                include_patterns: vec![],
            },
            grouping: GroupingConfig {
                enabled: true,
                groups: vec![
                    CommitGroup {
                        name: "Features".to_string(),
                        patterns: vec![
                            "^feat:".to_string(),
                            "^feature:".to_string(),
                            "add(?:s|ed|ing)?\\s".to_string(),
                            "new\\s".to_string(),
                        ],
                        description: Some("New features and enhancements".to_string()),
                    },
                    CommitGroup {
                        name: "Bug Fixes".to_string(),
                        patterns: vec![
                            "^fix:".to_string(),
                            "^bugfix:".to_string(),
                            "fix(?:es|ed|ing)?\\s".to_string(),
                            "bug(?:s)?\\s".to_string(),
                        ],
                        description: Some("Bug fixes and patches".to_string()),
                    },
                    CommitGroup {
                        name: "Documentation".to_string(),
                        patterns: vec![
                            "^docs:".to_string(),
                            "doc(?:s|umentation)?\\s".to_string(),
                            "readme".to_string(),
                        ],
                        description: None,
                    },
                    CommitGroup {
                        name: "Refactoring".to_string(),
                        patterns: vec![
                            "^refactor:".to_string(),
                            "refactor(?:s|ed|ing)?\\s".to_string(),
                            "cleanup".to_string(),
                        ],
                        description: None,
                    },
                    CommitGroup {
                        name: "Performance".to_string(),
                        patterns: vec![
                            "^perf:".to_string(),
                            "performance".to_string(),
                            "optimize".to_string(),
                        ],
                        description: None,
                    },
                    CommitGroup {
                        name: "Tests".to_string(),
                        patterns: vec![
                            "^test:".to_string(),
                            "^tests:".to_string(),
                            "test(?:s|ed|ing)?\\s".to_string(),
                        ],
                        description: None,
                    },
                    CommitGroup {
                        name: "Chores".to_string(),
                        patterns: vec![
                            "^chore:".to_string(),
                            "^ci:".to_string(),
                            "^build:".to_string(),
                        ],
                        description: Some("Maintenance tasks".to_string()),
                    },
                ],
            },
            templates: TemplateConfig {
                header: Some("# Changelog\n\nAll notable changes to this project will be documented in this file.\n".to_string()),
                footer: Some("\n---\nGenerated by [rcgen](https://github.com/yourusername/rcgen)".to_string()),
                commit_format: "- {message} ({hash} by {author})".to_string(),
                date_format: "%Y-%m-%d".to_string(),
            },
        }
    }
}
impl Config {
    pub fn load(path: &str) -> Result<Option<Self>> {
        let config_path = Path::new(path).join(".rcgen.toml");
        if !config_path.exists() {
            return Ok(None);
        }
        let content = fs::read_to_string(config_path)?;
        let config: Config =
            toml::from_str(&content).map_err(|e| RcgenError::Config(e.to_string()))?;
        Ok(Some(config))
    }
    pub fn save(&self, path: &str) -> Result<()> {
        let config_path = Path::new(path).join(".rcgen.toml");
        let content =
            toml::to_string_pretty(self).map_err(|e| RcgenError::Config(e.to_string()))?;
        fs::write(config_path, content)?;
        Ok(())
    }
    pub fn init(path: &str, force: bool) -> Result<()> {
        let config_path = Path::new(path).join(".rcgen.toml");
        if config_path.exists() && !force {
            return Err(RcgenError::Config(
                "Config file already exists. Use --force to overwrite.".to_string(),
            ));
        }
        let config = Config::default();
        config.save(path)
    }
}
