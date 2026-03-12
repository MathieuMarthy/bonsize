# treesize-cli 🐢

A CLI tool to display directory tree sizes.

## Description

`treesize-cli` allows you to quickly analyze and display the size of directories and files in a tree structure, making it easy to identify which files and folders are taking up the most disk space.


## Usage

```bash
treesize-cli [OPTIONS] [PATH]
```

### Arguments

- `[PATH]` - The directory path to analyze (default: current directory `.`)

### Options

- `-F, --file` - Show files in the output
- `-D, --directory` - Show only directories
- `-d, --depth <MAX_DEPTH>` - Maximum depth to traverse
- `-s, --sorted [<SORT>]` - Sort output (possible values: `asc`, `desc`)
- `--csv [<CSV>]` - Export results to CSV format
- `-c, --cache` - Use caching for improved performance
- `-q, --quiet` - Suppress non-essential output
- `-h, --help` - Print help information
- `-V, --version` - Print version information

### Examples

```bash
$ treesize-cli /folder
📂 - folder 109.15MB
  📂 - folder/videos 104.79MB
    📂 - folder/videos/2026 87.19MB
      📄 - folder/videos/2026/vacation_2026_02.mp4 87.19MB
    📂 - folder/videos/2025 17.60MB
      📄 - folder/videos/2025/Replay_2026-03-12_20-05-46.mp4 17.60MB
  📂 - folder/turtle pictures 4.36MB
    📄 - folder/turtle pictures/image_2.png 4.32MB
    📄 - folder/turtle pictures/image_1.jpg 40.00KB
  📄 - folder/letter.txt 4.00KB
```

```bash
# Show only directories, sorted by size
$ treesize-cli -D -s desc

# Limit depth and use cache
$ treesize-cli -d 3 -c
```


## Installation

### Windows

Download the Windows installer (`.exe`) from the [GitHub releases](https://github.com/MathieuMarthy/treesize-cli/releases).

### Debian/Ubuntu

Download the `.deb` package from the [GitHub releases](https://github.com/MathieuMarthy/treesize-cli/releases) and install it:

```bash
sudo dpkg -i treesize-cli_*.deb
```

### From source

If you have Rust installed, you can compile from source:

```bash
cargo install --path .
```


## License

This project is licensed under the WTFPL, see the [license file](./LICENSE) for more details.
