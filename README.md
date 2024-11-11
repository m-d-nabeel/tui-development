# JSON Editor

A terminal-based JSON editor built with Rust and Ratatui, designed for developers who want a quick and intuitive way to manage JSON files from the command line.

## Features

- **Intuitive Terminal User Interface (TUI)**: Easy-to-navigate interface built using Ratatui.
- **Real-time JSON Validation**: Instantly validates JSON as you edit.
- **Key-Value Pair Editing**: Effortlessly add, edit, and remove JSON key-value pairs.
- **Automatic JSON Formatting**: Ensures your JSON is consistently formatted.
- **Cross-Platform Support**: Runs on Linux, macOS, and Windows.

## Installation

Download the latest release from the [Releases page](https://github.com/m-d-nabeel/tui-development/releases), then extract and install the binary.

### Build from Source

Alternatively, you can build from source if you have Rust installed:

```bash
git clone https://github.com/your-username/json-editor.git
cd json-editor
cargo build --release
```

This will create an optimized binary in `target/release/json-editor`.

## Usage

Run the binary from your terminal:

```bash
./json-editor
```

### Updating Version

To update the version in your `Cargo.toml` file and create a new release tag, follow these steps:

1. **Update the `Cargo.toml` version to `0.1.1`**:

    ```bash
    # Update your Cargo.toml version to 0.1.1
    ```

2. **Commit the version bump**:

    ```bash
    git add Cargo.toml
    git commit -m "chore(release): bump version to 0.1.1"
    ```

3. **Create a new tag**:

    ```bash
    git tag -a v0.1.1 -m "Release v0.1.1

    Changes:
    - Improved documentation clarity
    - Fixed documentation typos
    - Enhanced release workflow configuration"
    ```

4. **Push the commits and tag**:

    ```bash
    git push origin main
    git push origin v0.1.1
    ```

## Contributing

Contributions are welcome! Please fork the repository and create a pull request with your proposed changes. 

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
