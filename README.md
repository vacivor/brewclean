# brewclean 🧹

A simple, fast, and efficient command-line tool written in Rust to manage and clean your Homebrew download cache.

Homebrew often leaves behind large downloaded archives (bottles) in its cache directory, which can consume significant disk space over time. `brewclean` helps you reclaim that space by listing or removing files based on their age.

## Features

-   **List files**: Preview which files are currently in your Homebrew cache.
-   **Time-based filtering**: Filter files older than a specific number of days.
-   **Clean up**: Remove cached files to free up disk space.
-   **Fast & Safe**: Built with Rust for performance, with a dedicated `ls` command for safe previews.

## Installation

### From Source

Ensure you have [Rust and Cargo](https://rustup.rs/) installed, then run:

```bash
# Clone the repository
git clone git@github.com/vacivor/brewclean.git
cd brewclean

# Build and install
cargo install --path .
```

### Via Cargo

```bash
cargo install brewclean
```

## Usage

### List Cached Files

To see all files currently in the Homebrew downloads cache:

```bash
brewclean ls
```

To see files older than 30 days:

```bash
brewclean ls --keep-days 30
```

### Clean Cache

To delete all files in the downloads cache:

```bash
brewclean clean
```

To delete only files older than 30 days (recommended):

```bash
brewclean clean --keep-days 30
```

## Options

### `ls` / `clean`

-   `--keep-days <DAYS>`: Keep files modified within the last N days. Files older than this will be listed or removed. (Default: 0, which targets all files).

## How It Works

`brewclean` targets the default Homebrew downloads directory on macOS:
`~/Library/Caches/Homebrew/downloads/`

It recursively scans this directory, checks the modification time of each file, and performs the requested action if the file's age exceeds the specified threshold.

## Safety First

-   **Preview Before Deleting**: Always run `brewclean ls` with your desired `--keep-days` first to ensure you're only removing what you intend to.
-   **Homebrew Cache**: Remember that cleaning the cache means Homebrew will need to re-download formulas or bottles if you need to reinstall them later.

## Requirements

-   **macOS**: Homebrew is primary used on macOS (Linux support not tested).
-   **Homebrew**: Installed and using the default cache path.

## License

This project is licensed under the MIT License.
