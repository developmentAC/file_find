# file_find

A fast file searching tool written in Rust. This tool works like `grep` but is useful for locating particular strings in files across a directory tree.

![logo](graphics/logo.png)

Date: 26 Nov 2025

Oliver Bonham-Carter

Email: obonhamcarter at allegheny.edu

A fast, colourized command-line tool written in Rust for recursively searching files in a directory tree by filename (using regular expressions) and optionally searching within those files for lines matching a second regular expression. Results are colour-highlighted for easy reading.

File-find, was designed for working primarily with web development projects, but can be used for any type of project.

## Features

- Recursively searches directories for files matching a filename regex
- Optionally searches inside matching files for lines matching a content regex
- colourized output for file paths and matching lines
- Displays search duration
- Helpful usage and extended help messages

## Setup

1. **Clone the repository** (if you haven't already):

   ```sh
   git clone <repo-url>
   cd file_find
   ```

2. **Build the project** (requires Rust and Cargo):

   ```sh
   cargo build --release
   ```

   The compiled binary will be in `target/release/file_find`.

## Usage

### Basic Usage

**Online Help:**

```sh
cargo run -- --bighelp
```

### Search for a specific filename

Find all files named `example.txt`:

```sh
cargo run -- example.txt
```

### Use a filename regex

Find all `.md` files:

```sh
cargo run -- ".*\.md$"
```

### Search for files and highlight lines containing a string/regex

Find all `.rs` files containing the word `main`:

```sh
cargo run -- ".*\.rs$" --search main
```

### Search all files for a term:

Find all `.rs` files containing the word `main`:

```sh
cargo run -- ".*\.rs$" --search "term"
```

or simply:

```sh
cargo run -- "\.rs"  --search "term"
```

### Combine filename and content search

Find all files with `data` in the name, containing the word `track`:

```sh
cargo run -- data --search track
```

### Search all files for a particular text

To search every file in the directory tree for a specific text or pattern, use a filename regex that matches all files (e.g., `.*`), combined with the `--search` option:

```sh
cargo run -- ".*" --search your_text_here
```

For example, to find all files containing the word `TODO`:

```sh
cargo run -- ".*" --search TODO
```

This will print the path of every file containing the text, and highlight each matching line.

### Extended Help

Show usage and regex tips:

```sh
cargo run -- --bighelp
```

## Simple Example

In the following command, we search for all Rust source files (`.rs`) in the current directory and its subdirectories that contain the word `expect`:

```sh 
cargo run -- ".rs" --search expect
```

The output will look like this:

```text
  ./src/main.rs
    Line 116:     let filename_regex = Regex::new(&args[1]).expect("Invalid filename regex");
    Line 120:             .map(|s| Regex::new(s).expect("Invalid search regex"))
  ./src/toml_extract.rs
    Line 26:     let content = fs::read_to_string(file_path).expect("Failed to read Cargo.toml file");
    Line 29:     let cargo_toml: CargoToml = from_str(&content).expect("Failed to parse Cargo.toml");
  Search completed in 36.25ms
```

Cool, right?!

## Notes

- The filename pattern is a regular expression (Rust regex syntax).
- The `--search` option is also a regular expression for matching lines inside files.
- Output is colourized for clarity.
- If a file cannot be read, a warning is shown.

---

## License

This project is licensed under the MIT License. See the LICENSE file for details.

## Contributing

Contributions are welcome! If you have ideas for improvements or want to add more features, feel free to open an issue or submit a pull request.

### A Work In Progress

Check back often to see the evolution of the project! This project is a work-in-progress. Updates will come periodically.

If you would like to contribute to this project, please do! For instance, if you see some low-hanging fruit or tasks that could add value to the project, I would love to have your insight.

Otherwise, please create an issue for bugs or errors. Since I am a teaching faculty member at Allegheny College, I may not have all the time necessary to quickly fix bugs. I welcome the Open Source Community to further the development of this project. Much thanks in advance.

If you appreciate this project, please consider clicking the project's Star button. :-)
