# RCGen - Git Changelog Generator

RCGen is a command-line tool for generating changelogs from Git repositories in Markdown, JSON, and plaintext formats. It's designed for developers who need informative and structured changelogs for their projects.

## Key Features

- **Three Output Formats**: Markdown, JSON, and Plaintext
- **Smart Grouping**: Automatically groups commits by type (feat, fix, docs, etc.)
- **Full Statistics**: In-depth analysis of the repository including bus factor, activity, and contributors
- **Flexible Filtering**: Filter commits by author, date, or message pattern
- **Diff Statistics**: Displays changed files, added lines, and deleted lines
- **Custom Configuration**: `.rcgen.toml` configuration file for customizing the output
- **Preview Mode**: Preview the changelog directly in the terminal
- **Conventional Commit Detection**: Supports Conventional Commits with scopes
- **Tag and Branch Information**: Displays the tags and branches associated with each commit

## Installation

### Using Cargo (from source)

```bash
# Clone repository
$ git clone https://github.com/neuxdotdev/rcgen
$ cd rcgen
# Build and install
$ cargo install --path .
```

## Quick Usage

```bash
# Generate Markdown changelog
$ rcgen gen --format md --output CHANGELOG.md
# Generate with grouping and statistics
$ rcgen gen --group --stats --limit 50
# Preview the last 10 commits
$ rcgen preview
# Show repository statistics
$ rcgen stats --detailed
```

## Complete Command

### `gen` - Generate Changelog

Generate a changelog from a Git repository.
**Options:**

| Option               | Description                                                   | Default / Values                                 |
| -------------------- | ------------------------------------------------------------- | ------------------------------------------------ |
| `--path <PATH>`      | Path to the Git repository                                    | `.`                                              |
| `--format <FORMAT>`  | Output format                                                 | `md` (`md`, `json`, `text`)                      |
| `--output <FILE>`    | Output file (stdout if not specified)                         | stdout                                           |
| `--limit <N>`        | Number of commits to include (0 for all)                      | `0`                                              |
| `--author <PATTERN>` | Filter by author name or email                                | —                                                |
| `--grep <PATTERN>`   | Filter commits by message pattern                             | —                                                |
| `--since <DATE>`     | Start date (`YYYY-MM-DD` or `"2 weeks ago"`)                  | —                                                |
| `--until <DATE>`     | Until date                                                    | —                                                |
| `--body`             | Include commit bodies                                         | false                                            |
| `--group`            | Group commits by type                                         | false                                            |
| `--sort <ORDER>`     | Sorting order                                                 | `reverse` (`chronological`, `reverse`, `author`) |
| `--no-merges`        | Exclude merge commits                                         | false                                            |
| `--stats`            | Include statistics                                            | false                                            |
| `--release`          | Generate release notes                                        | false                                            |
| `--diff-stats`       | Include diff statistics (file changes, insertions, deletions) | false                                            |

**Example:**

```bash
# Generate commits with author and date filters
$ rcgen gen --author "john" --since "2024-01-01" --until "2024-02-01"
# Generate JSON with all commits and statistics
$ rcgen gen --format json --limit 0 --stats
# Generate releases with grouping
$ rcgen gen --release --group --diff-stats
```

### `stats` - Repository Statistics

Show repository statistics.
**Options:**

| Option              | Description                | Default / Values              |
| ------------------- | -------------------------- | ----------------------------- |
| `--path <PATH>`     | Path to the Git repository | `.`                           |
| `--detailed`        | Show detailed statistics   | false                         |
| `--format <FORMAT>` | Output format              | `text` (`md`, `json`, `text`) |

**Example:**

```bash
# Basic statistics
$ rcgen stats
# Detailed statistics in JSON
$ rcgen stats --detailed --format json
# Statistics for a specific path
$ rcgen stats --path /path/to/repo
```

### `init` - Initialize Configuration

Initialize the `.$ rcgen.toml` configuration file.
**Options:**

| Option          | Description                               | Default / Values |
| --------------- | ----------------------------------------- | ---------------- |
| `--path <PATH>` | Path to the Git repository                | `.`              |
| `--force`       | Overwrite the existing configuration file | false            |

**Example:**

```bash
# Initialize the default configuration
$ rcgen init
# Overwrite the existing configuration
$ rcgen init --force
```

### `preview` - Preview Changelog

Preview the changelog in the terminal.
**Options:**

| Option          | Description                  | Default / Values |
| --------------- | ---------------------------- | ---------------- |
| `--path <PATH>` | Path to the Git repository   | `.`              |
| `--limit <N>`   | Number of commits to display | `10`             |

**Example:**

```bash
# Preview the last 10 commits
$ rcgen preview
# Preview the last 20 commits
$ rcgen preview --limit 20
```

### `diff` - Compare Revisions

Compare two revisions (not yet implemented).
**Options:**

| Option / Argument | Description                | Default / Values          |
| ----------------- | -------------------------- | ------------------------- |
| `--path <PATH>`   | Path to the Git repository | `.`                       |
| `from`            | Initial revision           | —                         |
| `to`              | Final revision             | `HEAD` (if not specified) |
| `--output <FILE>` | Output file                | stdout (if not specified) |

**Example:**

```bash
# Compare two tags
$ rcge diff v1.0.0 v2.0.0
# Compare commits with HEAD
$ rcgen diff abc123def
```

## Configuration

RCGen supports configuration through the `.rcgen.toml` file in the repository root. Run `rcgen init` to create the default configuration.

### Configuration Example

```toml
[filters]
exclude_authors = [ "bot@example.com" ]
exclude_patterns = [ "^Merge", "^Revert" ]
include_patterns = [ ]

[grouping]
enabled = true

  [[grouping.groups]]
  name = "Features"
  patterns = [ "^feat:", "adds?", "new" ]
  description = "New features and enhancements"

  [[grouping.groups]]
  name = "Bug Fixes"
  patterns = [ "^fix:", "bug" ]
  description = "Bug fixes and patches"

  [[grouping.groups]]
  name = "Documentation"
  patterns = [ "^docs:", "doc(?:s|umentation)?" ]
  description = "Documentation updates"

  [[grouping.groups]]
  name = "Refactoring"
  patterns = [ "^refactor:", "cleanup" ]
  description = "Code refactoring"

  [[grouping.groups]]
  name = "Performance"
  patterns = [ "^perf:", "optimize" ]
  description = "Performance improvements"

  [[grouping.groups]]
  name = "Tests"
  patterns = [ "^test:", "^tests:" ]
  description = "Test updates"

  [[grouping.groups]]
  name = "Chores"
  patterns = [ "^chore:", "^ci:", "^build:" ]
  description = "Maintenance tasks"

[output]
default_format = "md"
exclude_merges = true
include_body = false
include_diff_stats = true
max_commits = 100

[repository]
default_branch = "main"
tag_pattern = "v[0-9]*"
url = "https://github.com/username/repository"

[templates]
commit_format = "- {message} ({hash} by {author})"
date_format = "%Y-%m-%d"
footer = """

---
Generated by [RCGen](https://github.com/yoursname/rcgen)"""
header = """
# Changelog

All notable changes to this project will be documented in this file.
"""
```

### Configuration Explanation

#### `[repository]`

- `url`: Repository URL for reference
- `default_branch`: Default branch (default: "main")
- `tag_pattern`: Regex pattern to detect version tags

#### `[output]`

- `default_format`: Default output format [md, json, text]
- `include_body`: Include commit body
- `include_diff_stats`: Include diff statistics
- `exclude_merges`: Exclude merge commits
- `max_commits`: Maximum limit of commits processed

#### `[filters]`

- `exclude_authors`: List of authors to exclude
- `exclude_patterns`: Regex pattern to exclude commits
- `include_patterns`: Regex pattern to include commits (if empty, all commits are included)

#### `[grouping]`

- `enabled`: Enable grouping
- `groups`: List of commit groups with patterns and descriptions

#### `[templates]`

- `header`: Header template for output
- `footer`: Footer template for output
- `commit_format`: Format string for each commit (supports placeholders: {message}, {hash}, {author}, {date})
- `date_format`: Date format for output

## Output Examples

### Markdown Output

```markdown
# Changelog

## 2024-01-15

- feat: Add user authentication system (`abc123` by John Doe) - 3 files changed (+120 -10)
  > Implement JWT-based authentication
  > Add login and registration endpoints
- fix: Resolve memory leak in database connection (`def456` by Jane Smith) - 1 file changed (+5 -15)

## Statistics

- Total commits: 150
- Total authors: 5
- Files changed: 42
- Insertions: +5402
- Deletions: -1234
- Bus factor: 2.5

### Top Contributors

1. John Doe <john@example.com> - 42 commits (+1200 -234)
2. Jane Smith <jane@example.com> - 38 commits (+890 -456)
3. Bob Johnson <bob@example.com> - 25 commits (+650 -123)
```

### JSON Output

```json
{
  "commits": [
    {
      "hash": "abc123def456",
      "short_hash": "abc123",
      "author": {
        "name": "John Doe",
        "email": "john@example.com",
        "commits_count": 42
      },
      "date": "2024-01-15T10:30:00+00:00",
      "message": "feat: Add user authentication system\n\nImplement JWT-based authentication",
      "summary": "feat: Add user authentication system",
      "body": "Implement JWT-based authentication",
      "files_changed": ["src/auth.rs", "src/models.rs"],
      "insertions": 120,
      "deletions": 10,
      "is_merge": false,
      "tags": ["v1.2.0"],
      "branches": ["main"],
      "commit_type": "feat",
      "scope": null
    }
  ],
  "stats": {
    "total_commits": 150,
    "total_authors": 5,
    "first_commit": "2023-01-01T00:00:00+00:00",
    "last_commit": "2024-01-15T10:30:00+00:00",
    "period_days": 379,
    "commits_per_day": 0.4,
    "authors": [".."],
    "files_changed": 42,
    "total_insertions": 5402,
    "total_deletions": 1234,
    "bus_factor": 2.5,
    "commit_types": {
      "feats": 45,
      "fix": 32,
      "docs": 18,
      "refactor": 25,
      "chore": 30
    },
    "most_active_day": "2024-01-10",
    "most_active_hour": 14
  },
  "generated_at": "2024-01-15T12:00:00Z"
}
```

### Plaintext Outputs

```txt
CHANGELOG
==============================================================================
2024-01-15
----------
* feat: Add user authentication system [abc123] - John Doe (2024-01-15) | 3 files, +120/-10
> Implement JWT-based authentication
* fix: Resolve memory leak in database connection [def456] - Jane Smith (2024-01-15) | 1 file, +5/-15
STATISTICS
------------------------------------------------------------------------------
Total commits: 150
Total authors: 5
Files changed: 42
Total changes: +5402 / -1234
Bus factor: 2.
```
