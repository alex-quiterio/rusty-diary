# RustyDiary 📝

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
rusty-diary
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