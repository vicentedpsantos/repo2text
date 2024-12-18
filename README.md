# repo2text

[![Crates.io](https://img.shields.io/crates/v/repo2text.svg)](https://crates.io/crates/repo2text)

**repo2text** is a Rust CLI tool designed to transform the contents of a Git repository into a structured text file. This is particularly useful for uploading repository data into AI systems (like ChatGPT) for analysis, summarization, or debugging purposes.

## Features
- Recursively traverses a Git repository to extract file contents.
- Appends the file paths and contents into a single `.txt` file.
- Formats the output with clear delimiters for AI compatibility.
- Skips binary files and handles missing files gracefully.

## Installation

Install the tool using `cargo`:

```bash
cargo install repo2text
```

Or clone the repository and build from source:

```bash
git clone https://github.com/vicentedpsantos/repo2text.git
cd repo2text
cargo build --release
```

## Usage

Run `repo2text` with the following options:

```bash
repo2text --repo <REPO_PATH> --output <OUTPUT_FILE>
```

### Arguments:
- `--repo`: Path to the root of the Git repository.
- `--output`: Path to the output `.txt` file where the repository contents will be saved.

### Example

```bash
repo2text --repo ~/projects/faktur --output faktur.txt
```

This command will:
1. Traverse the repository at `~/projects/faktur`.
2. Read the contents of committed files.
3. Save the following structured data to `faktur.txt`:

### Output Format

The tool generates a `.txt` file where:
- Each file begins with a `FILE_PATH` line indicating its location in the repository.
- File contents are enclosed within a code block (` ```<language>` `...` ` ``` `).
- Sections are separated by `---` for easy parsing.

## Use Case: Uploading to AI Systems

The structured format makes it easy to share your repository with AI systems for:
- Code analysis or debugging.
- Generating documentation or summaries.
- Discussing code architecture or functionality.

Simply copy and paste sections of the output `.txt` file into your AI chat or upload the entire file if supported.

---

## Limitations
- Excludes uncommitted changes or ignored files.
- Binary files are skipped to prevent unreadable output.

## License

This project is licensed under the [MIT License](LICENSE).
