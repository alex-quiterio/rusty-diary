# RustyDiary 📝

[Cranes.io](https://crates.io/crates/rusty_diary)

RustyDiary is a Rust utility that helps you merge dated Markdown files into a single chronological log. It's perfect for combining daily notes, journal entries, or any date-stamped Markdown files.

## Features

- 📅 Automatically detects and merges files with date-based names (YYYY-MM-DD.md)
- ⬇️ Sorts entries in reverse chronological order (newest first)
- 🔄 Preserves existing content in the output file
- ⚙️ Configurable separators between entries
- 🛡️ Robust error handling and validation

## Installation

1. Ensure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/).

2. Clone this repository:
```bash
git clone [your-repository-url]
cd rusty-diary
```

3. Build the project:
```bash
cargo build --release
```

The compiled binary will be available in `target/release/rusty-diary`.

## Usage

Basic usage (current directory):
```bash
rusty-diary -h

A markdown diary with SQLite persistence

USAGE:
    rusty_diary [FLAGS] [OPTIONS] [directory]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Verbosity level

OPTIONS:
        --date-pattern <date-pattern>    Custom date pattern for files
        --db <db>                        Database file path

ARGS:
    <directory>    Directory containing markdown files
```

Specify a different directory:
```bash
rusty-diary /path/to/your/files
```

### File Naming Convention

Files should follow the pattern: `YYYY-MM-DD.md`

Examples:
- `2024-01-01.md`
- `2024-12-31.md`

### Configuration

The default configuration can be modified by creating a custom `Config` instance:

```rust
let config = Config {
    directory: PathBuf::from("your/path"),
    date_pattern: String::from(r"^\d{4}-\d{2}-\d{2}(\.md)?$"),
    output_filename: String::from("writing-log.md"),
    separator: String::from("\n---\n"),
};
```

## Development

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Dependencies

- `regex`: For pattern


## Opportunities for Future Improvement

RustyDiary has a solid foundation, and there are many exciting ways to expand its capabilities. We welcome contributions in any of these areas—whether you're fixing a small bug, adding a new feature, or improving documentation. Here are some ideas to get you started:

### 🤖 AI and Intelligence
- **LLM Integration**: Integrate large language models to generate summaries, insights, or trends from your diary data. Imagine getting weekly summaries or mood analysis automatically!

### 📅 Date and Format Flexibility
- **Flexible Date Formats**: Support various date formats beyond `YYYY-MM-DD.md`, including custom patterns, locale-specific formats, and mixed naming conventions.

### ⚙️ Configuration and Customization
- **Enhanced Configuration Options**: Add more output formats (JSON, HTML, PDF), metadata extraction, and advanced sorting options (by tags, topics, or custom criteria).

### 🗄️ Database and Search
- **Advanced Database Integration**: Implement sophisticated filtering and search capabilities, including full-text search, tag-based queries, and date range filtering.
- **Dashboard and Visualizations**: Create a statistics dashboard with visualizations of your writing habits, word counts over time, and sentiment trends.

### 🖥️ User Experience
- **Refined CLI/UX**: Improve command-line interface with better help text, interactive prompts, and progress indicators.
- **GUI Development**: Build a graphical interface for users who prefer not to use the command line, making diary management more accessible.

### 📚 Documentation
- **Comprehensive Documentation**: Expand guides for contributors, troubleshooting tips, usage examples, and best practices.
- **Tutorial Content**: Create step-by-step tutorials for common workflows and advanced use cases.

### 🧪 Testing and CI/CD
- **Automated Test Coverage**: Increase unit and integration test coverage to ensure reliability.
- **Continuous Integration**: Set up CI/CD workflows for automated testing, linting, and releases.

### 🔄 Interoperability
- **Import/Export Support**: Enable seamless import and export with popular journaling and note-taking apps like Day One, Obsidian, Joplin, Notion, and more.
- **Format Converters**: Build converters for common formats (Markdown variants, plain text, rich text).

### 🔌 Extensibility
- **Plugin System**: Develop a plugin architecture allowing users to extend functionality without modifying core code.
- **API Development**: Create APIs for programmatic access to diary operations and data.
- **Notification System**: Add support for reminders, writing streaks, and custom notifications.

### Getting Started with Contributing

Interested in working on any of these ideas? Check out the [contribution guidelines](CONTRIBUTING.md) (if available) or open an issue to discuss your ideas. We're excited to see what you'll build!
