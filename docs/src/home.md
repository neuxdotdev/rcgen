# RCGen - Git Changelog Generator

RCGen is a command-line tool for generating changelogs from Git repositories in Markdown, JSON, and plaintext formats. It is designed for developers who need informative and structured changelogs for their projects.

## Key Features

- **Three Output Formats**: Markdown, JSON, and Plaintext
- **Smart Grouping**: Automatically groups commits by type (feat, fix, docs, etc.)
- **Full Statistics**: In-depth repository analysis including bus factor, activity, and contributors
- **Flexible Filters**: Filter commits by author, date, or message pattern
- **Diff Statistics**: Displays changed files, added lines, and deleted lines
- **Custom Configuration**: `.rcgen.toml` configuration file to customize the output
- **Preview Mode**: Preview the changelog directly in the terminal
- **Conventional Commit Detection**: Supports Conventional Commits with scopes
- **Tag and Branch Information**: Displays the tags and branches associated with each commit

## Quick Start

### Installation

```bash
# From source using Cargo
$ cargo install --git https://github.com/neuxdotdev/rcgen

# Or clone first
$ git clone https://github.com/neuxdotdev/rcgen
$ rcgen cd
$ cargo install --path .
```

### Basic Usage

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

## Output Formats

RCGen supports three output formats:

1. Markdown (`--format md`): An easy-to-read format for documentation
2. JSON (`--format json`): For automated processing and integration
3. Plaintext (`--format text`): For simple terminal output

## Platform Support

RCGen supports the following platforms:
- Linux (x86_64, aarch64)
- macOS (x86_64, aarch64/Apple Silicon)
- Windows (x86_64)

## Navigation

- [Installation](./pages/install.md) - Complete installation guide
- [Configuration](./pages/configuration.md) - Detailed configuration guide
- [About Project](./pages/about.md) - Information about RCGen

## Sample Output

### Markdown

```markdown
# Changelog

## 2024-01-15
- feat: Add user authentication system (`abc123` by John Doe) - 3 files changed (+120 -10)
> Implement JWT-based authentication
> Add login and registration endpoints
```

### JSON
```json
{
  "commits": [
    {
      "hash": "abc123def456",
      "author": "John Doe",
      "message": "feat: Add user authentication system"
    }
  ]
}

```

## Contributions

See the [Contributing Guidelines](./meta/contributing.md) for information about contributing to RCGen.

## License

RCGen is released under the [AGPL-3.0 license](./meta/license.md).